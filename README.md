# now

`now` is a Rust-native local task runner that makes containerized toolchains feel invisible.
The long-term product goal is Docker-grade reproducibility with a CLI experience closer to
Vercel, Linear, and modern AI-native developer tools.

## Current Status

This repository currently contains the Phase 1 proof-of-concept:

- `now run <command...>` detects a sensible image for the current project.
- The current directory is mounted into `/workspace`.
- The command runs inside an ephemeral Docker-backed container.
- Container stdout/stderr streams back to the terminal.
- The container is removed after execution.

## Quickstart

```bash
cargo run -- run echo "hello from now"
```

In a Node project:

```bash
cargo run -- run npm install
```

To force an image:

```bash
cargo run -- run --image alpine:latest echo "hello"
```

## Roadmap

- Phase 1: Docker fallback proof-of-concept.
- Phase 2: `.now.yaml`, sequential execution, and polished terminal UI.
- Phase 3: Astro documentation site and SEO landing pages.
- Phase 4: binary distribution, installers, and native Linux OCI execution.

See `docs/ARCHITECTURE.md` and `docs/ROADMAP.md` for the full strategy canvas.
