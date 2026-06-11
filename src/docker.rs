use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use anyhow::{Context, Result, bail};
use bollard::{
    Docker,
    container::LogOutput,
    models::{ContainerCreateBody, HostConfig},
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, LogsOptionsBuilder,
        RemoveContainerOptionsBuilder, WaitContainerOptionsBuilder,
    },
};
use futures_util::StreamExt;
use uuid::Uuid;

use crate::ui;

#[derive(Debug, Clone)]
pub struct RunRequest {
    pub image: String,
    pub command: Vec<String>,
    pub workdir: PathBuf,
}

pub async fn run(request: RunRequest) -> Result<ExitCode> {
    let docker = Docker::connect_with_local_defaults()
        .context("failed to connect to a local Docker-compatible daemon")?;

    pull_image(&docker, &request.image).await?;

    let container_name = format!("now-{}", Uuid::new_v4().simple());
    let container_id = create_container(&docker, &container_name, &request).await?;

    let result = run_container(&docker, &container_id).await;
    let cleanup = remove_container(&docker, &container_id).await;

    if let Err(error) = cleanup {
        ui::warn(format!(
            "failed to remove container {container_id}: {error:#}"
        ));
    }

    result
}

async fn pull_image(docker: &Docker, image: &str) -> Result<()> {
    let (name, tag) = split_image(image);
    let options = CreateImageOptionsBuilder::default()
        .from_image(name)
        .tag(tag)
        .build();

    let spinner = ui::spinner(format!("preparing {image}"));
    let mut stream = docker.create_image(Some(options), None, None);

    while let Some(event) = stream.next().await {
        let event = event.with_context(|| format!("failed to pull {image}"))?;
        if let Some(status) = event.status {
            spinner.set_message(status);
        }
        if let Some(error) = event.error_detail.and_then(|detail| detail.message) {
            bail!("failed to pull {image}: {error}");
        }
    }

    spinner.finish_with_message(format!("ready {image}"));
    Ok(())
}

async fn create_container(docker: &Docker, name: &str, request: &RunRequest) -> Result<String> {
    let bind = format!(
        "{}:/workspace",
        normalize_mount_path(&request.workdir)?.display()
    );
    let config = ContainerCreateBody {
        image: Some(request.image.clone()),
        cmd: Some(request.command.clone()),
        working_dir: Some("/workspace".to_string()),
        tty: Some(false),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        host_config: Some(HostConfig {
            auto_remove: Some(false),
            binds: Some(vec![bind]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let options = CreateContainerOptionsBuilder::default().name(name).build();

    let created = docker
        .create_container(Some(options), config)
        .await
        .context("failed to create container")?;

    Ok(created.id)
}

async fn run_container(docker: &Docker, container_id: &str) -> Result<ExitCode> {
    docker
        .start_container(container_id, None)
        .await
        .context("failed to start container")?;

    stream_logs(docker, container_id).await?;

    let mut wait_stream = docker.wait_container(
        container_id,
        Some(WaitContainerOptionsBuilder::default().build()),
    );

    let exit = wait_stream
        .next()
        .await
        .context("container exited before Docker returned a status")?
        .context("failed while waiting for container")?;

    Ok(code_to_exit(exit.status_code))
}

async fn stream_logs(docker: &Docker, container_id: &str) -> Result<()> {
    let options = LogsOptionsBuilder::default()
        .follow(true)
        .stdout(true)
        .stderr(true)
        .tail("all")
        .build();

    let mut stream = docker.logs(container_id, Some(options));

    while let Some(output) = stream.next().await {
        match output.context("failed to stream container logs")? {
            LogOutput::StdOut { message } => print!("{}", String::from_utf8_lossy(&message)),
            LogOutput::StdErr { message } => eprint!("{}", String::from_utf8_lossy(&message)),
            LogOutput::Console { message } => print!("{}", String::from_utf8_lossy(&message)),
            LogOutput::StdIn { .. } => {}
        }
    }

    Ok(())
}

async fn remove_container(docker: &Docker, container_id: &str) -> Result<()> {
    let options = RemoveContainerOptionsBuilder::default()
        .force(true)
        .v(false)
        .build();

    docker
        .remove_container(container_id, Some(options))
        .await
        .context("failed to remove container")
}

fn split_image(image: &str) -> (&str, &str) {
    let last_slash = image.rfind('/');
    let last_colon = image.rfind(':');

    match (last_slash, last_colon) {
        (_, Some(colon)) if last_slash.is_none_or(|slash| colon > slash) => {
            (&image[..colon], &image[colon + 1..])
        }
        _ => (image, "latest"),
    }
}

fn normalize_mount_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("failed to resolve {}", path.display()))
}

fn code_to_exit(code: i64) -> ExitCode {
    match u8::try_from(code) {
        Ok(code) => ExitCode::from(code),
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use super::split_image;

    #[test]
    fn splits_tagged_images() {
        assert_eq!(split_image("alpine:latest"), ("alpine", "latest"));
        assert_eq!(split_image("node:20-alpine"), ("node", "20-alpine"));
        assert_eq!(
            split_image("ghcr.io/example/tool:v1"),
            ("ghcr.io/example/tool", "v1")
        );
    }

    #[test]
    fn keeps_registry_ports_without_tags() {
        assert_eq!(
            split_image("localhost:5000/example/tool"),
            ("localhost:5000/example/tool", "latest")
        );
    }
}
