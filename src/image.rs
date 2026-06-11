use std::path::Path;

pub fn infer_image(project_root: &Path) -> String {
    if project_root.join("package.json").exists() {
        return "node:20-alpine".to_string();
    }

    if project_root.join("pyproject.toml").exists()
        || project_root.join("requirements.txt").exists()
    {
        return "python:3.12-alpine".to_string();
    }

    if project_root.join("go.mod").exists() {
        return "golang:1.22-alpine".to_string();
    }

    if project_root.join("Cargo.toml").exists() {
        return "rust:1-alpine".to_string();
    }

    "alpine:latest".to_string()
}

pub fn image_for_toolchain(toolchain: &str) -> Option<String> {
    let (name, version) = toolchain.split_once('@').unwrap_or((toolchain, "latest"));

    match name {
        "node" => Some(format!("node:{version}-alpine")),
        "python" => Some(format!("python:{version}-alpine")),
        "go" | "golang" => Some(format!("golang:{version}-alpine")),
        "rust" => Some(format!("rust:{version}-alpine")),
        _ => None,
    }
}
