# Now CLI Roadmap

## Phase 1: Core Engine and Fallback Proof of Concept

- [x] Initialize Rust project.
- [x] Add core CLI shape.
- [x] Probe for Docker-compatible local execution.
- [x] Connect to Docker through `bollard`.
- [x] Pull an image through the Docker API.
- [x] Mount the current directory into a container.
- [x] Stream container stdout/stderr.
- [x] Remove containers after command completion.

## Phase 2: Configuration and UI Transformation

- [ ] Define the stable `.now.yaml` schema.
- [ ] Resolve `now <alias>` commands from project config.
- [ ] Execute sequential multi-step pipelines.
- [ ] Add rich `indicatif` pull/setup progress.
- [ ] Add `now init` with `inquire` stack selection.
- [ ] Improve actionable error messages.

## Phase 3: Web Presence and Documentation

- [ ] Scaffold Astro, React, and Tailwind site.
- [ ] Build dark-mode landing page aesthetic.
- [ ] Add animated terminal hero component.
- [ ] Add interactive WebGL or motion background.
- [ ] Write Quickstart, Installation, YAML Reference, and CLI Reference docs.
- [ ] Deploy to GitHub Pages from GitHub Actions.

## Phase 4: Distribution, Native Execution, and Polish

- [ ] Add release matrix for macOS, Linux, and Windows binaries.
- [ ] Create Homebrew tap workflow.
- [ ] Add `curl | bash` installer.
- [ ] Implement native Linux OCI image pulling.
- [ ] Add `crun`/`youki` execution path.
- [ ] Add integration tests for execution backends.
