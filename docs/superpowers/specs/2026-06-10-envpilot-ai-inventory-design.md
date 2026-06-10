# EnvPilot AI Inventory MVP Design

Date: 2026-06-10

## Goal

EnvPilot MVP provides a cross-platform AI development environment inventory for macOS and Windows. The first version focuses on read-only discovery and analysis for AI tool and agent ecosystems. It validates the scan core through a CLI and exposes the same results in a simple desktop console.

## Confirmed Scope

In scope:

- Scan installed and partially configured AI tools:
  - Codex
  - Claude Code
  - Gemini CLI
  - OpenCode
- Detect executables, versions, installation evidence, config files, config directories, MCP servers, skills, agents, plugins/extensions, and instruction files.
- Provide CLI commands for scanning, diagnostics, and config inspection.
- Provide a read-only Tauri desktop console.
- Provide read-only config file viewing and analysis with secret redaction.
- Support macOS and Windows scanning paths through platform-specific path resolvers.

Out of scope for MVP:

- Installing, uninstalling, upgrading, or switching tool versions.
- Editing or automatically fixing configuration files.
- Displaying raw secrets.
- Executing commands from discovered config files.
- Cloud sync, accounts, or team policy management.

## Architecture

EnvPilot uses a Rust workspace with a shared scan core, CLI, and Tauri desktop shell.

```text
EnvPilot/
- crates/envpilot-core
  - detector interface
  - scan scheduler
  - inventory data model
  - path resolvers
  - config parsers
  - secret redaction
  - diagnostics
- crates/envpilot-cli
  - envpilot scan
  - envpilot doctor
  - envpilot config list
  - envpilot config inspect
- src-tauri
  - Tauri commands that call envpilot-core
- ui
  - Dashboard
  - Tool Detail
  - Config Viewer
  - Warnings
  - Settings
```

The scan core must not depend on UI code. CLI and desktop both call the same core APIs so scan behavior does not fork between surfaces.

## Data Model

The top-level output is `InventoryReport`.

```text
InventoryReport
- schema_version
- generated_at
- platform
- host_summary
- tools: Vec<AiToolRecord>
- warnings: Vec<Diagnostic>
```

Each detected tool is represented as `AiToolRecord`.

```text
AiToolRecord
- id: codex | claude-code | gemini-cli | opencode
- name
- status: installed | partially_detected | not_found | error
- version
- executables
- install_sources
- config_files
- config_dirs
- mcp_servers
- skills
- agents
- instructions
- warnings
- evidence
```

The model keeps observable evidence separate from conclusions. Evidence can include resolved executable paths, command output summaries, discovered config keys, filesystem metadata, and parse results.

## Scan Rules

- Scanning is read-only.
- A single detector failure must not stop the full scan.
- Secrets must be redacted. Sensitive keys include token, api_key, password, secret, cookie, authorization, and close variants.
- Config commands and MCP server commands are inspected as data only. MVP must not execute discovered config commands.
- Detection should prefer observable paths and parser results over assumptions.
- macOS and Windows path rules are isolated behind platform-specific resolvers.
- Tool-specific config paths and formats must be verified during implementation before being hard-coded.

## CLI Behavior

```bash
envpilot scan
```

Prints a table with tool name, status, version, config count, and warning count.

```bash
envpilot scan --format json
```

Prints the full `InventoryReport` as JSON.

```bash
envpilot doctor
```

Prints diagnostics grouped by severity, including missing executables, failed version commands, parse failures, missing referenced paths, duplicate MCP servers, and suspicious config fields.

```bash
envpilot config list
```

Lists discovered config files with tool, file type, path, size, modified time, parse status, and warning count.

```bash
envpilot config inspect <tool-id> --file <path>
envpilot config inspect <tool-id> --format json
```

Shows a redacted config summary or JSON detail. It never prints raw secrets.

## Desktop Behavior

The desktop app is read-only in MVP.

Pages:

- `Dashboard`: tool cards for Codex, Claude Code, Gemini CLI, and OpenCode with status, version, and warning count.
- `Tool Detail`: executables, install evidence, config files, MCP servers, skills, agents, instructions, and tool-specific warnings.
- `Config Viewer`: redacted file view, structured summary, parser diagnostics, references, and risk flags.
- `Warnings`: cross-tool warning list grouped by severity and tool.
- `Settings`: scan settings only, such as extra scan paths and whether workspace instruction files are included.

## Config Viewer And Analyzer

The Config Viewer adds read-only configuration analysis to the MVP.

It shows:

- File path, source tool, file type, file size, and modified time.
- Parse status and parser error details.
- Structured summaries for MCP servers, commands, args, env field names, skills, agents, plugins, and instruction references.
- Redacted file content when the user opens a file view.
- Diagnostics for duplicate config entries, missing referenced paths, non-existent MCP commands, invalid structure, and suspicious sensitive fields.

It does not:

- Save edits.
- Auto-fix files.
- Show raw secrets.
- Execute commands found inside config files.

## Detector Flow

Each detector follows the same flow:

```text
1. executable detection
2. version detection
3. config discovery
4. config parsing
5. ecosystem discovery
6. diagnostics
```

Executable detection checks PATH and platform-specific known locations. Version detection uses safe version commands with timeouts. Config discovery scans known user config directories and optionally the current workspace. Config parsing supports JSON, TOML, YAML, and Markdown where relevant. Unknown formats keep metadata and warnings.

## Initial Detectors

`codex`:

- CLI executable.
- Codex config directory.
- AGENTS.md.
- Skills.
- MCP configuration.

`claude-code`:

- CLI executable.
- CLAUDE.md.
- Agents, commands, hooks, and settings config entry points.

`gemini-cli`:

- CLI executable.
- GEMINI.md.
- Settings/config files.
- MCP-related config where supported.

`opencode`:

- CLI executable.
- User and project config.
- Agents and command config entry points.

Implementation must verify the current real config paths and formats for these tools before finalizing detector constants.

## Verification Strategy

Core tests:

- Fixture tests with temporary home and workspace directories.
- Parser tests for JSON, TOML, YAML, Markdown, and secret redaction.
- Detector tests with fake PATH, fake executables, and fake config trees.
- Warning tests for malformed config, missing commands, missing referenced paths, duplicate MCP server names, and failed version commands.

CLI tests:

- JSON snapshot tests for `scan --format json`.
- Smoke tests for `scan`, `doctor`, `config list`, and `config inspect`.

Desktop tests:

- Tauri command tests for scan and config inspection.
- Minimal UI tests for dashboard, tool detail, config viewer, and warnings.

Manual verification:

- macOS local scan.
- Windows scan on a separate Windows environment. macOS results must not be treated as proof of Windows correctness.

## Open Decisions For Implementation

- Exact Rust/Tauri scaffold and package layout.
- Frontend framework selection for Tauri UI.
- Exact current config paths and schemas for Codex, Claude Code, Gemini CLI, and OpenCode.
- Whether the first implementation includes workspace scan by default or requires an explicit CLI flag.
