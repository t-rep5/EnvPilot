# Errors

Command failures and integration errors.

---

## [ERR-20260610-001] cargo_missing

**Logged**: 2026-06-10T15:56:35+08:00
**Priority**: high
**Status**: pending
**Area**: infra

### Summary
`cargo check --workspace` could not run because Cargo is not available through the current PATH, and the rustup shim has no default toolchain configured.

### Error
```text
zsh:1: command not found: cargo
error: rustup could not choose a version of cargo to run, because one wasn't specified explicitly, and no default is configured.
```

### Context
- Command attempted: `cargo check --workspace`
- Workspace: `/Users/jack/Documents/My_File/app/EnvPilot`
- The project is being scaffolded as a Rust workspace, so Cargo is required for verification.

### Suggested Fix
Install Rust with rustup, run `rustup default stable`, or provide a shell PATH/toolchain that includes Cargo, then rerun `cargo check --workspace` and `cargo test --workspace`.

### Metadata
- Reproducible: yes
- Related Files: Cargo.toml

---

## [ERR-20260610-002] tauri_cli_download_timeout

**Logged**: 2026-06-10T16:33:00+08:00
**Priority**: high
**Status**: pending
**Area**: infra

### Summary
Installing `tauri-cli` with Cargo failed because the crate download timed out.

### Error
```text
error: failed to download from `https://static.crates.io/crates/tauri-cli/2.11.2/download`
Caused by:
  [28] Timeout was reached
```

### Context
- Command attempted: `/Users/jack/.cargo/bin/cargo install tauri-cli --version '^2.0.0' --locked`
- The Rust workspace builds and tests pass before packaging.
- A DMG build requires a Tauri CLI entrypoint such as `cargo tauri build --bundles dmg` or `npm run tauri:build`.

### Suggested Fix
Retry when network access to crates.io is stable, use a configured Cargo mirror, or install the Tauri CLI through a working package-manager path.

### Metadata
- Reproducible: unknown
- Related Files: ui/package.json, src-tauri/tauri.conf.json

---
