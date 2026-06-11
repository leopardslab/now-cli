# Now CLI Architecture Canvas

## Vision

`now` removes local development environment friction for AI-native builders. It gives users
mathematically reproducible execution consistency without asking them to understand Docker,
virtual environments, shell paths, or host-level toolchain installation.

The product promise is simple:

> Code more. Configure never.

## Target User

`now` is designed for the "vibe engineer": a goal-oriented developer who wants to ship features,
delegate boilerplate to AI tools, and avoid host machine configuration. This user values:

- Instant feedback.
- Minimal setup.
- Beautiful terminal UX.
- Thoughtful errors.
- Reproducible execution without infrastructure ceremony.

## Core Principles

- **Zero host dependencies**: the installed surface area should collapse toward a single `now`
  binary.
- **Ephemeral contexts**: toolchains exist only for the lifetime of the command.
- **Invisible containers**: Docker/OCI images are implementation details, not user concepts.
- **Progressive execution**: choose the fastest supported backend for the host OS.
- **Polished feedback**: progress, prompts, and errors should feel intentionally designed.

## Execution Backends

### Tier 1: Native OCI for Linux and CI

Linux should eventually use direct OCI image pulls plus a daemonless runtime such as `crun` or
`youki`. This provides the fastest path and matches the zero-daemon philosophy.

### Tier 2: Docker API fallback for macOS and Windows

macOS and Windows need a virtualization layer. The first production-compatible backend should
probe for Docker Desktop, OrbStack, or Colima sockets, then talk directly to the Docker API with
`bollard`. Users never invoke `docker` directly.

## CLI Workflows

### Implicit execution

```bash
now run npm install
```

`now` detects project files, selects a default image, mounts the current directory, runs the
command, streams output, and destroys the container.

### Configured pipelines

`.now.yaml` will define named commands and sequential multi-image pipelines:

```yaml
commands:
  test:
    image: node:20-alpine
    run: npm test
  deploy:
    steps:
      - image: golang:1.22-alpine
        run: go build ./...
      - image: amazon/aws-cli:latest
        run: aws s3 sync dist/ s3://example-bucket
```

### Interactive shells

```bash
now shell python@3.9
```

Shell sessions should open inside isolated containers and vanish on exit.

## Rust Crate Stack

- `clap`: CLI parsing.
- `tokio`: async runtime for Docker calls and streaming.
- `bollard`: Docker API client for the fallback backend.
- `serde` and `serde_yaml`: `.now.yaml` parsing.
- `indicatif`: progress bars and spinners.
- `inquire`: interactive onboarding prompts.
- `console` and `owo-colors`: semantic terminal styling.

## Repository Layout

```text
src/
  cli.rs        # command-line interface
  config.rs     # .now.yaml data model
  detect.rs     # backend/socket detection
  docker.rs     # Docker fallback executor
  image.rs      # project-to-image inference
  main.rs       # app entrypoint
  ui.rs         # terminal styling helpers
docs/
  ARCHITECTURE.md
  ROADMAP.md
```
