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
