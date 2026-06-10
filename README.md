# EnvPilot

EnvPilot is a cross-platform development environment inventory tool for macOS and Windows. The MVP focuses on read-only AI tool and agent ecosystem scanning for Codex, Claude Code, Gemini CLI, and OpenCode.

## MVP Scope

- Scan AI tool installation evidence, versions, config files, MCP servers, skills, agents, and instruction files.
- Provide CLI inventory and diagnostics.
- Provide a read-only Tauri desktop console.
- Provide redacted config viewing and analysis.

The MVP does not install, uninstall, upgrade, switch versions, edit config files, execute discovered config commands, or display raw secrets.

## Development

```bash
cargo check --workspace
cargo test --workspace
cargo run -p envpilot -- scan
cargo run -p envpilot -- scan --format json
cargo run -p envpilot -- doctor
cargo run -p envpilot -- config list
```

## Design

See `docs/superpowers/specs/2026-06-10-envpilot-ai-inventory-design.md`.
