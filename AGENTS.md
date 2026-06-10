# EnvPilot Agent Instructions

## Project Goal

EnvPilot MVP is a read-only AI development environment inventory for macOS and Windows.

## Architecture

- `crates/envpilot-core`: data model, detector interface, scan orchestration, config parsing, redaction, diagnostics.
- `crates/envpilot-cli`: CLI wrapper over `envpilot-core`.
- `src-tauri`: Tauri commands that call `envpilot-core`.
- `ui`: read-only desktop frontend.

## Verification

Run the narrowest relevant command first:

```bash
cargo test -p envpilot-core
cargo test -p envpilot --test cli_tests
cargo check --workspace
cargo test --workspace
```

For frontend changes, also run the configured UI build once dependencies are installed:

```bash
cd ui && npm run build
```

## Constraints

- MVP scanning is read-only.
- Do not add install, uninstall, upgrade, or version-switch behavior unless explicitly requested.
- Do not print or store raw secrets.
- Do not execute commands discovered inside config files.
- Verify current tool config paths and formats before hard-coding detector constants.
- Keep CLI and desktop behavior backed by the same `envpilot-core` APIs.
