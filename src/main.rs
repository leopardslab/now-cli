mod cli;
mod config;
mod detect;
mod docker;
mod image;
mod ui;

use std::{
    env,
    process::{Command as HostCommand, ExitCode, Stdio},
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use cli::{Cli, Commands};
use config::{Command as ConfigCommand, Step};
use detect::Backend;
use docker::RunRequest;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { image, command } => run_command(image, command).await,
        Commands::Shell { toolchain } => shell_command(toolchain).await,
        Commands::Init => init_command(),
    }
}

async fn run_command(image: Option<String>, command: Vec<String>) -> Result<ExitCode> {
    let project_root = env::current_dir().context("failed to read current directory")?;

    if image.is_none()
        && command.len() == 1
        && let Some(config) = config::load_from(project_root.join(".now.yaml"))?
        && let Some(config_command) = config.commands.get(&command[0])
    {
        return run_config_command(config_command.clone(), project_root).await;
    }

    let backend = detect::detect_backend();
    let image = image.unwrap_or_else(|| image::infer_image(&project_root));

    announce_backend(&backend, &image);

    docker::run(RunRequest {
        image,
        command,
        workdir: project_root,
    })
    .await
}

async fn run_config_command(
    command: ConfigCommand,
    project_root: std::path::PathBuf,
) -> Result<ExitCode> {
    let steps = match command {
        ConfigCommand::Single(step) => vec![step],
        ConfigCommand::Pipeline { steps } => steps,
    };

    for step in steps {
        let status = run_step(step, project_root.clone()).await?;
        if status != ExitCode::SUCCESS {
            return Ok(status);
        }
    }

    Ok(ExitCode::SUCCESS)
}

async fn run_step(step: Step, project_root: std::path::PathBuf) -> Result<ExitCode> {
    ui::info(format!("running {}", step.run));

    if step.host {
        let status = HostCommand::new("sh")
            .arg("-lc")
            .arg(&step.run)
            .current_dir(&project_root)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("failed to run host command: {}", step.run))?;

        return Ok(status_to_exit_code(status.code()));
    }

    docker::run(RunRequest {
        image: step.image.context("container steps require an image")?,
        command: vec!["sh".to_string(), "-lc".to_string(), step.run],
        workdir: project_root,
    })
    .await
}

async fn shell_command(toolchain: String) -> Result<ExitCode> {
    let _ = image::image_for_toolchain(&toolchain)
        .with_context(|| format!("unknown toolchain selector: {toolchain}"))?;
    bail!("now shell is planned but not wired yet; use `now run --image <image> <command>` for now")
}

fn init_command() -> Result<ExitCode> {
    let cwd = env::current_dir().context("failed to read current directory")?;
    let config_path = cwd.join(".now.yaml");

    if config_path.exists() {
        bail!(".now.yaml already exists");
    }

    std::fs::write(
        &config_path,
        "commands:\n  test:\n    image: node:20-alpine\n    run: npm test\n",
    )
    .with_context(|| format!("failed to write {}", config_path.display()))?;

    let _ = config::load_from(&config_path)?;
    ui::success(format!("created {}", config_path.display()));

    Ok(ExitCode::SUCCESS)
}

fn announce_backend(backend: &Backend, image: &str) {
    match backend {
        Backend::Docker { hint } => {
            ui::info(format!("using Docker fallback via {hint}; image {image}"))
        }
    }
}

fn status_to_exit_code(code: Option<i32>) -> ExitCode {
    match code.and_then(|code| u8::try_from(code).ok()) {
        Some(code) => ExitCode::from(code),
        None => ExitCode::FAILURE,
    }
}
