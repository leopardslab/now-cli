use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Backend {
    Docker { hint: String },
}

pub fn detect_backend() -> Backend {
    if let Ok(host) = std::env::var("DOCKER_HOST") {
        return Backend::Docker {
            hint: format!("DOCKER_HOST={host}"),
        };
    }

    if let Some(socket) = detect_unix_socket() {
        return Backend::Docker {
            hint: socket.display().to_string(),
        };
    }

    #[cfg(windows)]
    {
        return Backend::Docker {
            hint: r"\\.\pipe\docker_engine".to_string(),
        };
    }

    Backend::Docker {
        hint: "local Docker defaults".to_string(),
    }
}

#[cfg(unix)]
fn detect_unix_socket() -> Option<PathBuf> {
    let mut candidates = vec![PathBuf::from("/var/run/docker.sock")];

    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        candidates.push(home.join(".docker/run/docker.sock"));
        candidates.push(home.join(".colima/default/docker.sock"));
        candidates.push(home.join(".orbstack/run/docker.sock"));
    }

    candidates.into_iter().find(|path| path.exists())
}

#[cfg(not(unix))]
fn detect_unix_socket() -> Option<PathBuf> {
    None
}
