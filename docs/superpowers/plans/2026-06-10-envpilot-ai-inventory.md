# EnvPilot AI Inventory Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the EnvPilot MVP as a Rust scan core with a CLI and a read-only Tauri desktop console for AI tool inventory and config analysis.

**Architecture:** The project is a Rust workspace. `envpilot-core` owns the data model, detector contract, config parsing, redaction, and scan orchestration. `envpilot-cli` exposes the core through automation-friendly commands. Tauri calls the same core and renders a read-only dashboard, tool details, warnings, and config analysis.

**Tech Stack:** Rust, Cargo workspace, clap, serde, serde_json, toml, serde_yaml, pulldown-cmark or markdown metadata scanning, assert_cmd, insta or similar snapshots, Tauri, TypeScript frontend.

---

## File Structure

Create:

- `Cargo.toml`: workspace root.
- `crates/envpilot-core/Cargo.toml`: core crate manifest.
- `crates/envpilot-core/src/lib.rs`: public core module exports.
- `crates/envpilot-core/src/model.rs`: inventory, tool, config, diagnostic, and evidence types.
- `crates/envpilot-core/src/redaction.rs`: recursive secret redaction for parsed config values and text.
- `crates/envpilot-core/src/config.rs`: config file metadata, parser dispatch, structured config summaries.
- `crates/envpilot-core/src/path.rs`: platform-aware home/workspace/path resolver abstractions.
- `crates/envpilot-core/src/detector/mod.rs`: detector trait and registry.
- `crates/envpilot-core/src/detector/generic.rs`: reusable executable, version, and file discovery helpers.
- `crates/envpilot-core/src/detector/codex.rs`: Codex detector.
- `crates/envpilot-core/src/detector/claude.rs`: Claude Code detector.
- `crates/envpilot-core/src/detector/gemini.rs`: Gemini CLI detector.
- `crates/envpilot-core/src/detector/opencode.rs`: OpenCode detector.
- `crates/envpilot-core/src/scan.rs`: scan orchestration.
- `crates/envpilot-core/tests/model_tests.rs`: serialization and model contract tests.
- `crates/envpilot-core/tests/redaction_tests.rs`: redaction tests.
- `crates/envpilot-core/tests/config_tests.rs`: parser and config analyzer tests.
- `crates/envpilot-core/tests/detector_tests.rs`: fixture-based detector tests.
- `crates/envpilot-cli/Cargo.toml`: CLI crate manifest.
- `crates/envpilot-cli/src/main.rs`: CLI command definitions and output.
- `crates/envpilot-cli/tests/cli_tests.rs`: CLI smoke tests.
- `src-tauri/Cargo.toml`: Tauri Rust manifest.
- `src-tauri/src/lib.rs`: Tauri command module.
- `src-tauri/src/main.rs`: Tauri entrypoint.
- `ui/package.json`: frontend package scripts.
- `ui/src/main.tsx`: frontend entrypoint.
- `ui/src/App.tsx`: app shell.
- `ui/src/api.ts`: Tauri command API wrapper.
- `ui/src/types.ts`: frontend mirror of inventory types.
- `ui/src/pages/Dashboard.tsx`: dashboard page.
- `ui/src/pages/ToolDetail.tsx`: tool detail page.
- `ui/src/pages/ConfigViewer.tsx`: config viewer page.
- `ui/src/pages/Warnings.tsx`: warnings page.
- `ui/src/pages/Settings.tsx`: scan settings page.
- `README.md`: project purpose and local development commands.
- `AGENTS.md`: project-specific agent instructions, scripts, and verification commands.

Avoid creating installer, updater, version-switching, cloud sync, or config editing files in this MVP.

## Task 1: Initialize The Rust Workspace

**Files:**

- Create: `Cargo.toml`
- Create: `crates/envpilot-core/Cargo.toml`
- Create: `crates/envpilot-core/src/lib.rs`
- Create: `crates/envpilot-cli/Cargo.toml`
- Create: `crates/envpilot-cli/src/main.rs`

- [ ] **Step 1: Create the workspace manifest**

Write `Cargo.toml`:

```toml
[workspace]
members = [
  "crates/envpilot-core",
  "crates/envpilot-cli",
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT"
version = "0.1.0"
```

- [ ] **Step 2: Create the core crate manifest**

Write `crates/envpilot-core/Cargo.toml`:

```toml
[package]
name = "envpilot-core"
edition.workspace = true
license.workspace = true
version.workspace = true

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
thiserror = "1"
toml = "0.8"
which = "6"

[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3: Create the initial core library**

Write `crates/envpilot-core/src/lib.rs`:

```rust
// Modules are added task-by-task as the MVP is implemented.
```

- [ ] **Step 4: Create the CLI manifest**

Write `crates/envpilot-cli/Cargo.toml`:

```toml
[package]
name = "envpilot"
edition.workspace = true
license.workspace = true
version.workspace = true

[[bin]]
name = "envpilot"
path = "src/main.rs"

[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
envpilot-core = { path = "../envpilot-core" }
serde_json = "1"
```

- [ ] **Step 5: Create a temporary CLI entrypoint**

Write `crates/envpilot-cli/src/main.rs`:

```rust
fn main() {
    println!("envpilot");
}
```

- [ ] **Step 6: Run workspace check**

Run:

```bash
cargo check --workspace
```

Expected: Cargo resolves dependencies and both crates compile.

## Task 2: Define Inventory Data Model

**Files:**

- Create: `crates/envpilot-core/src/model.rs`
- Create: `crates/envpilot-core/tests/model_tests.rs`

- [ ] **Step 1: Write model serialization tests**

Write `crates/envpilot-core/tests/model_tests.rs`:

```rust
use envpilot_core::model::{
    AiToolRecord, Diagnostic, DiagnosticSeverity, Evidence, InventoryReport, PlatformInfo, ToolId,
    ToolStatus,
};

#[test]
fn inventory_report_serializes_stable_tool_ids_and_statuses() {
    let report = InventoryReport {
        schema_version: "1".to_string(),
        generated_at: "2026-06-10T00:00:00Z".to_string(),
        platform: PlatformInfo {
            os: "macos".to_string(),
            arch: "aarch64".to_string(),
            hostname: None,
        },
        tools: vec![AiToolRecord {
            id: ToolId::Codex,
            name: "Codex".to_string(),
            status: ToolStatus::Installed,
            version: Some("1.2.3".to_string()),
            executables: vec!["/usr/local/bin/codex".to_string()],
            install_sources: vec!["homebrew".to_string()],
            config_files: vec![],
            config_dirs: vec![],
            mcp_servers: vec![],
            skills: vec![],
            agents: vec![],
            instructions: vec![],
            warnings: vec![],
            evidence: vec![Evidence {
                kind: "path".to_string(),
                summary: "found executable".to_string(),
                value: Some("/usr/local/bin/codex".to_string()),
            }],
        }],
        warnings: vec![Diagnostic {
            severity: DiagnosticSeverity::Info,
            code: "scan.completed".to_string(),
            message: "scan completed".to_string(),
            tool_id: None,
            path: None,
        }],
    };

    let json = serde_json::to_value(report).expect("report serializes");
    assert_eq!(json["tools"][0]["id"], "codex");
    assert_eq!(json["tools"][0]["status"], "installed");
    assert_eq!(json["warnings"][0]["severity"], "info");
}
```

- [ ] **Step 2: Run the model test and verify it fails**

Run:

```bash
cargo test -p envpilot-core --test model_tests
```

Expected: fails because `model` types are not implemented.

- [ ] **Step 3: Implement the model types**

Write `crates/envpilot-core/src/model.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryReport {
    pub schema_version: String,
    pub generated_at: String,
    pub platform: PlatformInfo,
    pub tools: Vec<AiToolRecord>,
    pub warnings: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiToolRecord {
    pub id: ToolId,
    pub name: String,
    pub status: ToolStatus,
    pub version: Option<String>,
    pub executables: Vec<String>,
    pub install_sources: Vec<String>,
    pub config_files: Vec<ConfigFileRecord>,
    pub config_dirs: Vec<String>,
    pub mcp_servers: Vec<McpServerRecord>,
    pub skills: Vec<ResourceRecord>,
    pub agents: Vec<ResourceRecord>,
    pub instructions: Vec<ResourceRecord>,
    pub warnings: Vec<Diagnostic>,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ToolId {
    Codex,
    ClaudeCode,
    GeminiCli,
    Opencode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Installed,
    PartiallyDetected,
    NotFound,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigFileRecord {
    pub path: String,
    pub tool_id: ToolId,
    pub file_type: ConfigFileType,
    pub exists: bool,
    pub parse_status: ParseStatus,
    pub summary: Option<ConfigSummary>,
    pub redacted_preview: Option<String>,
    pub warnings: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigFileType {
    Json,
    Toml,
    Yaml,
    Markdown,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatus {
    Parsed,
    NotParsed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ConfigSummary {
    pub mcp_server_count: usize,
    pub skill_count: usize,
    pub agent_count: usize,
    pub instruction_count: usize,
    pub sensitive_field_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpServerRecord {
    pub name: String,
    pub source_path: String,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub env_field_names: Vec<String>,
    pub warnings: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceRecord {
    pub name: String,
    pub path: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub tool_id: Option<ToolId>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Evidence {
    pub kind: String,
    pub summary: String,
    pub value: Option<String>,
}
```

- [ ] **Step 4: Run the model test**

Run:

```bash
cargo test -p envpilot-core --test model_tests
```

Expected: passes.

- [ ] **Step 5: Commit**

Run:

```bash
git add Cargo.toml crates/envpilot-core crates/envpilot-cli
git commit -m "feat: add inventory data model"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 3: Implement Secret Redaction

**Files:**

- Create: `crates/envpilot-core/src/redaction.rs`
- Create: `crates/envpilot-core/tests/redaction_tests.rs`

- [ ] **Step 1: Write redaction tests**

Write `crates/envpilot-core/tests/redaction_tests.rs`:

```rust
use envpilot_core::redaction::{redact_json_value, redact_text};
use serde_json::json;

#[test]
fn redact_json_value_hides_sensitive_keys_recursively() {
    let value = json!({
        "mcpServers": {
            "demo": {
                "command": "demo-server",
                "env": {
                    "API_KEY": "abc123",
                    "NORMAL_FLAG": "true"
                }
            }
        },
        "token": "secret-token"
    });

    let redacted = redact_json_value(value);
    assert_eq!(redacted["token"], "[REDACTED]");
    assert_eq!(redacted["mcpServers"]["demo"]["env"]["API_KEY"], "[REDACTED]");
    assert_eq!(redacted["mcpServers"]["demo"]["env"]["NORMAL_FLAG"], "true");
}

#[test]
fn redact_text_hides_assignment_style_secrets() {
    let text = "api_key = \"abc123\"\nnormal = \"visible\"\nAuthorization: Bearer xyz";
    let redacted = redact_text(text);
    assert!(redacted.contains("api_key = \"[REDACTED]\""));
    assert!(redacted.contains("normal = \"visible\""));
    assert!(redacted.contains("Authorization: [REDACTED]"));
    assert!(!redacted.contains("abc123"));
    assert!(!redacted.contains("Bearer xyz"));
}
```

- [ ] **Step 2: Run redaction tests and verify they fail**

Run:

```bash
cargo test -p envpilot-core --test redaction_tests
```

Expected: fails because redaction functions are not implemented.

- [ ] **Step 3: Implement redaction**

Write `crates/envpilot-core/src/redaction.rs`:

```rust
use serde_json::{Map, Value};

const REDACTED: &str = "[REDACTED]";

pub fn redact_json_value(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(redact_object(map)),
        Value::Array(items) => Value::Array(items.into_iter().map(redact_json_value).collect()),
        other => other,
    }
}

fn redact_object(map: Map<String, Value>) -> Map<String, Value> {
    map.into_iter()
        .map(|(key, value)| {
            if is_sensitive_key(&key) {
                (key, Value::String(REDACTED.to_string()))
            } else {
                (key, redact_json_value(value))
            }
        })
        .collect()
}

pub fn redact_text(input: &str) -> String {
    input
        .lines()
        .map(redact_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn redact_line(line: &str) -> String {
    let lower = line.to_ascii_lowercase();
    if !SENSITIVE_KEYS.iter().any(|key| lower.contains(key)) {
        return line.to_string();
    }

    if let Some(index) = line.find('=') {
        let key = line[..index].trim();
        if is_sensitive_key(key) {
            return format!("{}= \"{}\"", line[..index].trim_end(), REDACTED);
        }
    }

    if let Some(index) = line.find(':') {
        let key = line[..index].trim();
        if is_sensitive_key(key) {
            return format!("{}: {}", key, REDACTED);
        }
    }

    line.to_string()
}

pub fn is_sensitive_key(key: &str) -> bool {
    let normalized = key
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_ascii_lowercase()
        .replace('-', "_");

    SENSITIVE_KEYS
        .iter()
        .any(|sensitive| normalized.contains(sensitive))
}

const SENSITIVE_KEYS: &[&str] = &[
    "token",
    "api_key",
    "apikey",
    "password",
    "passwd",
    "secret",
    "cookie",
    "authorization",
    "access_key",
    "private_key",
];
```

- [ ] **Step 4: Run redaction tests**

Run:

```bash
cargo test -p envpilot-core --test redaction_tests
```

Expected: passes.

- [ ] **Step 5: Commit**

Run:

```bash
git add crates/envpilot-core/src/redaction.rs crates/envpilot-core/tests/redaction_tests.rs
git commit -m "feat: add secret redaction"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 4: Implement Config Parsing And Analysis

**Files:**

- Create: `crates/envpilot-core/src/config.rs`
- Create: `crates/envpilot-core/tests/config_tests.rs`

- [ ] **Step 1: Write config parser tests**

Write `crates/envpilot-core/tests/config_tests.rs`:

```rust
use envpilot_core::config::{analyze_config_text, detect_file_type};
use envpilot_core::model::{ConfigFileType, ToolId};

#[test]
fn detect_file_type_uses_extension() {
    assert_eq!(detect_file_type("settings.json"), ConfigFileType::Json);
    assert_eq!(detect_file_type("config.toml"), ConfigFileType::Toml);
    assert_eq!(detect_file_type("mcp.yaml"), ConfigFileType::Yaml);
    assert_eq!(detect_file_type("AGENTS.md"), ConfigFileType::Markdown);
    assert_eq!(detect_file_type("config"), ConfigFileType::Unknown);
}

#[test]
fn analyze_json_config_redacts_sensitive_values_and_counts_mcp_servers() {
    let text = r#"{
      "mcpServers": {
        "filesystem": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-filesystem"],
          "env": {
            "API_KEY": "secret",
            "VISIBLE": "yes"
          }
        }
      }
    }"#;

    let record = analyze_config_text(ToolId::Codex, "/tmp/config.json", text).expect("parsed");
    let summary = record.summary.expect("summary");
    assert_eq!(summary.mcp_server_count, 1);
    assert_eq!(summary.sensitive_field_names, vec!["API_KEY"]);
    assert!(record.redacted_preview.unwrap().contains("[REDACTED]"));
}

#[test]
fn analyze_invalid_json_returns_failed_record() {
    let record = analyze_config_text(ToolId::Codex, "/tmp/config.json", "{").expect("record");
    assert_eq!(record.parse_status.to_string(), "failed");
    assert_eq!(record.warnings.len(), 1);
}
```

- [ ] **Step 2: Add `Display` for `ParseStatus`**

Modify `crates/envpilot-core/src/model.rs` and add:

```rust
impl std::fmt::Display for ParseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ParseStatus::Parsed => "parsed",
            ParseStatus::NotParsed => "not_parsed",
            ParseStatus::Failed => "failed",
        };
        f.write_str(value)
    }
}
```

- [ ] **Step 3: Run config tests and verify they fail**

Run:

```bash
cargo test -p envpilot-core --test config_tests
```

Expected: fails because config parser functions are not implemented.

- [ ] **Step 4: Implement config analysis**

Write `crates/envpilot-core/src/config.rs`:

```rust
use crate::model::{
    ConfigFileRecord, ConfigFileType, ConfigSummary, Diagnostic, DiagnosticSeverity, ParseStatus,
    ToolId,
};
use crate::redaction::{is_sensitive_key, redact_json_value, redact_text};
use serde_json::Value;

pub fn detect_file_type(path: &str) -> ConfigFileType {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".json") {
        ConfigFileType::Json
    } else if lower.ends_with(".toml") {
        ConfigFileType::Toml
    } else if lower.ends_with(".yaml") || lower.ends_with(".yml") {
        ConfigFileType::Yaml
    } else if lower.ends_with(".md") {
        ConfigFileType::Markdown
    } else {
        ConfigFileType::Unknown
    }
}

pub fn analyze_config_text(
    tool_id: ToolId,
    path: &str,
    text: &str,
) -> Result<ConfigFileRecord, String> {
    let file_type = detect_file_type(path);
    match file_type {
        ConfigFileType::Json => analyze_json(tool_id, path, text),
        ConfigFileType::Toml | ConfigFileType::Yaml | ConfigFileType::Markdown => {
            Ok(ConfigFileRecord {
                path: path.to_string(),
                tool_id,
                file_type,
                exists: true,
                parse_status: ParseStatus::NotParsed,
                summary: Some(ConfigSummary {
                    sensitive_field_names: collect_sensitive_field_names_from_text(text),
                    ..ConfigSummary::default()
                }),
                redacted_preview: Some(redact_text(text)),
                warnings: vec![],
            })
        }
        ConfigFileType::Unknown => Ok(ConfigFileRecord {
            path: path.to_string(),
            tool_id,
            file_type,
            exists: true,
            parse_status: ParseStatus::NotParsed,
            summary: None,
            redacted_preview: None,
            warnings: vec![Diagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "config.unsupported_type".to_string(),
                message: "unsupported config file type".to_string(),
                tool_id: None,
                path: Some(path.to_string()),
            }],
        }),
    }
}

fn analyze_json(tool_id: ToolId, path: &str, text: &str) -> Result<ConfigFileRecord, String> {
    let parsed: Value = match serde_json::from_str(text) {
        Ok(value) => value,
        Err(error) => {
            return Ok(ConfigFileRecord {
                path: path.to_string(),
                tool_id: tool_id.clone(),
                file_type: ConfigFileType::Json,
                exists: true,
                parse_status: ParseStatus::Failed,
                summary: None,
                redacted_preview: None,
                warnings: vec![Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: "config.parse_failed".to_string(),
                    message: error.to_string(),
                    tool_id: Some(tool_id),
                    path: Some(path.to_string()),
                }],
            });
        }
    };

    let summary = summarize_json(&parsed);
    let redacted = redact_json_value(parsed);
    let redacted_preview = serde_json::to_string_pretty(&redacted).map_err(|e| e.to_string())?;

    Ok(ConfigFileRecord {
        path: path.to_string(),
        tool_id,
        file_type: ConfigFileType::Json,
        exists: true,
        parse_status: ParseStatus::Parsed,
        summary: Some(summary),
        redacted_preview: Some(redacted_preview),
        warnings: vec![],
    })
}

fn summarize_json(value: &Value) -> ConfigSummary {
    let mut summary = ConfigSummary::default();
    if let Some(servers) = value.get("mcpServers").and_then(|v| v.as_object()) {
        summary.mcp_server_count = servers.len();
    }
    collect_sensitive_field_names(value, &mut summary.sensitive_field_names);
    summary.sensitive_field_names.sort();
    summary.sensitive_field_names.dedup();
    summary
}

fn collect_sensitive_field_names(value: &Value, names: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if is_sensitive_key(key) {
                    names.push(key.clone());
                }
                collect_sensitive_field_names(value, names);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_sensitive_field_names(item, names);
            }
        }
        _ => {}
    }
}

fn collect_sensitive_field_names_from_text(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in text.lines() {
        let key = line
            .split(['=', ':'])
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if is_sensitive_key(key) {
            names.push(key.to_string());
        }
    }
    names.sort();
    names.dedup();
    names
}
```

- [ ] **Step 5: Run config tests**

Run:

```bash
cargo test -p envpilot-core --test config_tests
```

Expected: passes.

- [ ] **Step 6: Commit**

Run:

```bash
git add crates/envpilot-core/src/config.rs crates/envpilot-core/src/model.rs crates/envpilot-core/tests/config_tests.rs
git commit -m "feat: analyze config files"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 5: Implement Path Resolver And Detector Contract

**Files:**

- Create: `crates/envpilot-core/src/path.rs`
- Create: `crates/envpilot-core/src/detector/mod.rs`
- Create: `crates/envpilot-core/src/detector/generic.rs`
- Modify: `crates/envpilot-core/src/lib.rs`

- [ ] **Step 1: Write detector contract tests**

Create `crates/envpilot-core/tests/detector_tests.rs` with:

```rust
use envpilot_core::detector::{Detector, DetectorContext};
use envpilot_core::model::{AiToolRecord, ToolId, ToolStatus};
use envpilot_core::path::PathResolver;
use std::path::PathBuf;

struct FakeDetector;

impl Detector for FakeDetector {
    fn id(&self) -> ToolId {
        ToolId::Codex
    }

    fn detect(&self, _context: &DetectorContext) -> AiToolRecord {
        AiToolRecord {
            id: ToolId::Codex,
            name: "Codex".to_string(),
            status: ToolStatus::NotFound,
            version: None,
            executables: vec![],
            install_sources: vec![],
            config_files: vec![],
            config_dirs: vec![],
            mcp_servers: vec![],
            skills: vec![],
            agents: vec![],
            instructions: vec![],
            warnings: vec![],
            evidence: vec![],
        }
    }
}

#[test]
fn detector_contract_returns_tool_record() {
    let resolver = PathResolver::new(PathBuf::from("/tmp/home"), Some(PathBuf::from("/tmp/work")));
    let context = DetectorContext { resolver };
    let record = FakeDetector.detect(&context);
    assert_eq!(record.id, ToolId::Codex);
    assert_eq!(record.status, ToolStatus::NotFound);
}
```

- [ ] **Step 2: Run detector tests and verify they fail**

Run:

```bash
cargo test -p envpilot-core --test detector_tests
```

Expected: fails because detector and path modules are not implemented.

- [ ] **Step 3: Implement path resolver**

Write `crates/envpilot-core/src/path.rs`:

```rust
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PathResolver {
    home_dir: PathBuf,
    workspace_dir: Option<PathBuf>,
}

impl PathResolver {
    pub fn new(home_dir: PathBuf, workspace_dir: Option<PathBuf>) -> Self {
        Self {
            home_dir,
            workspace_dir,
        }
    }

    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    pub fn workspace_dir(&self) -> Option<&Path> {
        self.workspace_dir.as_deref()
    }

    pub fn home_path(&self, relative: &str) -> PathBuf {
        self.home_dir.join(relative)
    }

    pub fn workspace_path(&self, relative: &str) -> Option<PathBuf> {
        self.workspace_dir.as_ref().map(|dir| dir.join(relative))
    }
}
```

- [ ] **Step 4: Implement detector trait and generic helper module**

Write `crates/envpilot-core/src/detector/mod.rs`:

```rust
pub mod generic;

use crate::model::{AiToolRecord, ToolId};
use crate::path::PathResolver;

#[derive(Debug, Clone)]
pub struct DetectorContext {
    pub resolver: PathResolver,
}

pub trait Detector {
    fn id(&self) -> ToolId;
    fn detect(&self, context: &DetectorContext) -> AiToolRecord;
}
```

Write `crates/envpilot-core/src/detector/generic.rs`:

```rust
use crate::model::{AiToolRecord, Evidence, ToolId, ToolStatus};

pub fn empty_record(id: ToolId, name: &str) -> AiToolRecord {
    AiToolRecord {
        id,
        name: name.to_string(),
        status: ToolStatus::NotFound,
        version: None,
        executables: vec![],
        install_sources: vec![],
        config_files: vec![],
        config_dirs: vec![],
        mcp_servers: vec![],
        skills: vec![],
        agents: vec![],
        instructions: vec![],
        warnings: vec![],
        evidence: vec![],
    }
}

pub fn path_evidence(path: impl Into<String>, summary: &str) -> Evidence {
    Evidence {
        kind: "path".to_string(),
        summary: summary.to_string(),
        value: Some(path.into()),
    }
}
```

- [ ] **Step 5: Run detector tests**

Run:

```bash
cargo test -p envpilot-core --test detector_tests
```

Expected: passes.

- [ ] **Step 6: Commit**

Run:

```bash
git add crates/envpilot-core/src/path.rs crates/envpilot-core/src/detector crates/envpilot-core/tests/detector_tests.rs
git commit -m "feat: add detector contract"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 6: Implement Initial Tool Detectors With Verified Constants

**Files:**

- Create: `crates/envpilot-core/src/detector/codex.rs`
- Create: `crates/envpilot-core/src/detector/claude.rs`
- Create: `crates/envpilot-core/src/detector/gemini.rs`
- Create: `crates/envpilot-core/src/detector/opencode.rs`
- Modify: `crates/envpilot-core/src/detector/mod.rs`
- Modify: `crates/envpilot-core/tests/detector_tests.rs`

- [ ] **Step 1: Verify current config paths before coding detector constants**

Use official docs or local installed tool inspection for Codex, Claude Code, Gemini CLI, and OpenCode. Record exact config paths and version command behavior in implementation notes before hard-coding constants.

Commands to prefer for local inspection:

```bash
which codex || true
which claude || true
which gemini || true
which opencode || true
find "$HOME" -maxdepth 3 \( -name AGENTS.md -o -name CLAUDE.md -o -name GEMINI.md \) 2>/dev/null | head -50
```

Expected: observable evidence for executable names and config footprints. If a tool is not installed locally, use fixture-driven detector constants and leave a warning in implementation notes that live verification remains pending.

- [ ] **Step 2: Add detector tests for workspace instruction files**

Append to `crates/envpilot-core/tests/detector_tests.rs`:

```rust
use envpilot_core::detector::{CodexDetector, ClaudeDetector, GeminiDetector};
use std::fs;

#[test]
fn codex_detector_finds_workspace_agents_md() {
    let temp = tempfile::tempdir().expect("tempdir");
    let workspace = temp.path().join("work");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::write(workspace.join("AGENTS.md"), "# Rules").expect("agents");

    let resolver = PathResolver::new(temp.path().join("home"), Some(workspace));
    let context = DetectorContext { resolver };

    let record = CodexDetector.detect(&context);
    assert_eq!(record.id, ToolId::Codex);
    assert_eq!(record.instructions.len(), 1);
    assert_eq!(record.status, ToolStatus::PartiallyDetected);
}

#[test]
fn claude_detector_finds_workspace_claude_md() {
    let temp = tempfile::tempdir().expect("tempdir");
    let workspace = temp.path().join("work");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::write(workspace.join("CLAUDE.md"), "# Claude").expect("claude");

    let resolver = PathResolver::new(temp.path().join("home"), Some(workspace));
    let context = DetectorContext { resolver };

    let record = ClaudeDetector.detect(&context);
    assert_eq!(record.id, ToolId::ClaudeCode);
    assert_eq!(record.instructions.len(), 1);
    assert_eq!(record.status, ToolStatus::PartiallyDetected);
}

#[test]
fn gemini_detector_finds_workspace_gemini_md() {
    let temp = tempfile::tempdir().expect("tempdir");
    let workspace = temp.path().join("work");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::write(workspace.join("GEMINI.md"), "# Gemini").expect("gemini");

    let resolver = PathResolver::new(temp.path().join("home"), Some(workspace));
    let context = DetectorContext { resolver };

    let record = GeminiDetector.detect(&context);
    assert_eq!(record.id, ToolId::GeminiCli);
    assert_eq!(record.instructions.len(), 1);
    assert_eq!(record.status, ToolStatus::PartiallyDetected);
}
```

- [ ] **Step 3: Run detector tests and verify they fail**

Run:

```bash
cargo test -p envpilot-core --test detector_tests
```

Expected: fails because concrete detectors are not implemented.

- [ ] **Step 4: Implement concrete detectors**

Write `crates/envpilot-core/src/detector/codex.rs`:

```rust
use super::generic::{empty_record, path_evidence};
use super::{Detector, DetectorContext};
use crate::model::{ResourceRecord, ToolId, ToolStatus};

pub struct CodexDetector;

impl Detector for CodexDetector {
    fn id(&self) -> ToolId {
        ToolId::Codex
    }

    fn detect(&self, context: &DetectorContext) -> crate::model::AiToolRecord {
        let mut record = empty_record(ToolId::Codex, "Codex");
        if let Some(path) = context.resolver.workspace_path("AGENTS.md") {
            if path.exists() {
                record.instructions.push(ResourceRecord {
                    name: "AGENTS.md".to_string(),
                    path: path.display().to_string(),
                    source: "workspace".to_string(),
                });
                record.evidence.push(path_evidence(path.display().to_string(), "found workspace AGENTS.md"));
            }
        }
        if !record.instructions.is_empty() {
            record.status = ToolStatus::PartiallyDetected;
        }
        record
    }
}
```

Write `crates/envpilot-core/src/detector/claude.rs`:

```rust
use super::generic::{empty_record, path_evidence};
use super::{Detector, DetectorContext};
use crate::model::{ResourceRecord, ToolId, ToolStatus};

pub struct ClaudeDetector;

impl Detector for ClaudeDetector {
    fn id(&self) -> ToolId {
        ToolId::ClaudeCode
    }

    fn detect(&self, context: &DetectorContext) -> crate::model::AiToolRecord {
        let mut record = empty_record(ToolId::ClaudeCode, "Claude Code");
        if let Some(path) = context.resolver.workspace_path("CLAUDE.md") {
            if path.exists() {
                record.instructions.push(ResourceRecord {
                    name: "CLAUDE.md".to_string(),
                    path: path.display().to_string(),
                    source: "workspace".to_string(),
                });
                record.evidence.push(path_evidence(path.display().to_string(), "found workspace CLAUDE.md"));
            }
        }
        if !record.instructions.is_empty() {
            record.status = ToolStatus::PartiallyDetected;
        }
        record
    }
}
```

Write `crates/envpilot-core/src/detector/gemini.rs`:

```rust
use super::generic::{empty_record, path_evidence};
use super::{Detector, DetectorContext};
use crate::model::{ResourceRecord, ToolId, ToolStatus};

pub struct GeminiDetector;

impl Detector for GeminiDetector {
    fn id(&self) -> ToolId {
        ToolId::GeminiCli
    }

    fn detect(&self, context: &DetectorContext) -> crate::model::AiToolRecord {
        let mut record = empty_record(ToolId::GeminiCli, "Gemini CLI");
        if let Some(path) = context.resolver.workspace_path("GEMINI.md") {
            if path.exists() {
                record.instructions.push(ResourceRecord {
                    name: "GEMINI.md".to_string(),
                    path: path.display().to_string(),
                    source: "workspace".to_string(),
                });
                record.evidence.push(path_evidence(path.display().to_string(), "found workspace GEMINI.md"));
            }
        }
        if !record.instructions.is_empty() {
            record.status = ToolStatus::PartiallyDetected;
        }
        record
    }
}
```

Write `crates/envpilot-core/src/detector/opencode.rs`:

```rust
use super::generic::empty_record;
use super::{Detector, DetectorContext};
use crate::model::ToolId;

pub struct OpencodeDetector;

impl Detector for OpencodeDetector {
    fn id(&self) -> ToolId {
        ToolId::Opencode
    }

    fn detect(&self, _context: &DetectorContext) -> crate::model::AiToolRecord {
        empty_record(ToolId::Opencode, "OpenCode")
    }
}
```

Modify `crates/envpilot-core/src/detector/mod.rs`:

```rust
pub mod claude;
pub mod codex;
pub mod gemini;
pub mod generic;
pub mod opencode;

pub use claude::ClaudeDetector;
pub use codex::CodexDetector;
pub use gemini::GeminiDetector;
pub use opencode::OpencodeDetector;

use crate::model::{AiToolRecord, ToolId};
use crate::path::PathResolver;

#[derive(Debug, Clone)]
pub struct DetectorContext {
    pub resolver: PathResolver,
}

pub trait Detector {
    fn id(&self) -> ToolId;
    fn detect(&self, context: &DetectorContext) -> AiToolRecord;
}

pub fn default_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(CodexDetector),
        Box::new(ClaudeDetector),
        Box::new(GeminiDetector),
        Box::new(OpencodeDetector),
    ]
}
```

- [ ] **Step 5: Run detector tests**

Run:

```bash
cargo test -p envpilot-core --test detector_tests
```

Expected: passes.

- [ ] **Step 6: Commit**

Run:

```bash
git add crates/envpilot-core/src/detector crates/envpilot-core/tests/detector_tests.rs
git commit -m "feat: add initial ai tool detectors"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 7: Implement Scan Orchestration

**Files:**

- Create: `crates/envpilot-core/src/scan.rs`
- Create: `crates/envpilot-core/tests/scan_tests.rs`

- [ ] **Step 1: Write scan orchestration test**

Write `crates/envpilot-core/tests/scan_tests.rs`:

```rust
use envpilot_core::{scan_inventory, ScanOptions};
use envpilot_core::model::{ToolId, ToolStatus};
use std::fs;

#[test]
fn scan_inventory_runs_default_detectors() {
    let temp = tempfile::tempdir().expect("tempdir");
    let workspace = temp.path().join("work");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::write(workspace.join("AGENTS.md"), "# Rules").expect("agents");

    let report = scan_inventory(ScanOptions {
        home_dir: temp.path().join("home"),
        workspace_dir: Some(workspace),
    });

    assert_eq!(report.schema_version, "1");
    assert_eq!(report.tools.len(), 4);
    let codex = report.tools.iter().find(|tool| tool.id == ToolId::Codex).expect("codex");
    assert_eq!(codex.status, ToolStatus::PartiallyDetected);
}
```

- [ ] **Step 2: Run scan test and verify it fails**

Run:

```bash
cargo test -p envpilot-core --test scan_tests
```

Expected: fails because `scan_inventory` is not implemented.

- [ ] **Step 3: Implement scan orchestration**

Write `crates/envpilot-core/src/scan.rs`:

```rust
use crate::detector::{default_detectors, DetectorContext};
use crate::model::{Diagnostic, InventoryReport, PlatformInfo};
use crate::path::PathResolver;
use chrono::Utc;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub home_dir: PathBuf,
    pub workspace_dir: Option<PathBuf>,
}

pub fn scan_inventory(options: ScanOptions) -> InventoryReport {
    let resolver = PathResolver::new(options.home_dir, options.workspace_dir);
    let context = DetectorContext { resolver };
    let tools = default_detectors()
        .into_iter()
        .map(|detector| detector.detect(&context))
        .collect();

    InventoryReport {
        schema_version: "1".to_string(),
        generated_at: Utc::now().to_rfc3339(),
        platform: PlatformInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: None,
        },
        tools,
        warnings: Vec::<Diagnostic>::new(),
    }
}
```

- [ ] **Step 4: Run scan test**

Run:

```bash
cargo test -p envpilot-core --test scan_tests
```

Expected: passes.

- [ ] **Step 5: Commit**

Run:

```bash
git add crates/envpilot-core/src/scan.rs crates/envpilot-core/tests/scan_tests.rs
git commit -m "feat: orchestrate inventory scans"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 8: Implement CLI Commands

**Files:**

- Modify: `crates/envpilot-cli/src/main.rs`
- Create: `crates/envpilot-cli/tests/cli_tests.rs`
- Modify: `crates/envpilot-cli/Cargo.toml`

- [ ] **Step 1: Add CLI test dependencies**

Modify `crates/envpilot-cli/Cargo.toml`:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

- [ ] **Step 2: Write CLI smoke tests**

Write `crates/envpilot-cli/tests/cli_tests.rs`:

```rust
use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn scan_json_outputs_inventory_report() {
    let mut cmd = Command::cargo_bin("envpilot").expect("binary");
    cmd.args(["scan", "--format", "json"])
        .assert()
        .success()
        .stdout(contains("\"schema_version\""))
        .stdout(contains("\"tools\""));
}

#[test]
fn doctor_outputs_diagnostics_heading() {
    let mut cmd = Command::cargo_bin("envpilot").expect("binary");
    cmd.arg("doctor")
        .assert()
        .success()
        .stdout(contains("EnvPilot Doctor"));
}

#[test]
fn config_list_outputs_heading() {
    let mut cmd = Command::cargo_bin("envpilot").expect("binary");
    cmd.args(["config", "list"])
        .assert()
        .success()
        .stdout(contains("Config Files"));
}
```

- [ ] **Step 3: Run CLI tests and verify they fail**

Run:

```bash
cargo test -p envpilot --test cli_tests
```

Expected: fails because commands are not implemented.

- [ ] **Step 4: Implement CLI commands**

Write `crates/envpilot-cli/src/main.rs`:

```rust
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use envpilot_core::{scan_inventory, ScanOptions};

#[derive(Debug, Parser)]
#[command(name = "envpilot")]
#[command(about = "Development environment inventory for AI tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    Doctor,
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    List,
    Inspect {
        tool_id: String,
        #[arg(long)]
        file: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { format } => run_scan(format),
        Command::Doctor => run_doctor(),
        Command::Config { command } => run_config(command),
    }
}

fn run_scan(format: OutputFormat) -> Result<()> {
    let report = current_report();
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Table => {
            println!("Tool\tStatus\tVersion\tConfigs\tWarnings");
            for tool in report.tools {
                println!(
                    "{}\t{:?}\t{}\t{}\t{}",
                    tool.name,
                    tool.status,
                    tool.version.unwrap_or_else(|| "-".to_string()),
                    tool.config_files.len(),
                    tool.warnings.len()
                );
            }
        }
    }
    Ok(())
}

fn run_doctor() -> Result<()> {
    let report = current_report();
    println!("EnvPilot Doctor");
    for warning in report.warnings {
        println!("{:?}\t{}\t{}", warning.severity, warning.code, warning.message);
    }
    Ok(())
}

fn run_config(command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::List => {
            let report = current_report();
            println!("Config Files");
            for tool in report.tools {
                for config in tool.config_files {
                    println!("{}\t{}\t{:?}", tool.name, config.path, config.parse_status);
                }
            }
        }
        ConfigCommand::Inspect { tool_id, file, format } => {
            println!("Config Inspect");
            println!("tool_id={tool_id}");
            if let Some(file) = file {
                println!("file={file}");
            }
            println!("format={format:?}");
        }
    }
    Ok(())
}

fn current_report() -> envpilot_core::model::InventoryReport {
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let workspace_dir = std::env::current_dir().ok();
    scan_inventory(ScanOptions {
        home_dir,
        workspace_dir,
    })
}
```

- [ ] **Step 5: Run CLI tests**

Run:

```bash
cargo test -p envpilot --test cli_tests
```

Expected: passes.

- [ ] **Step 6: Commit**

Run:

```bash
git add crates/envpilot-cli
git commit -m "feat: add envpilot cli commands"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 9: Scaffold Read-Only Tauri Desktop Shell

**Files:**

- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/src/main.rs`
- Create: `ui/package.json`
- Create: `ui/src/main.tsx`
- Create: `ui/src/App.tsx`
- Create: `ui/src/api.ts`
- Create: `ui/src/types.ts`
- Create: `ui/src/pages/Dashboard.tsx`
- Create: `ui/src/pages/ToolDetail.tsx`
- Create: `ui/src/pages/ConfigViewer.tsx`
- Create: `ui/src/pages/Warnings.tsx`
- Create: `ui/src/pages/Settings.tsx`

- [ ] **Step 1: Add Tauri crate to workspace**

Modify root `Cargo.toml` members:

```toml
[workspace]
members = [
  "crates/envpilot-core",
  "crates/envpilot-cli",
  "src-tauri",
]
resolver = "2"
```

- [ ] **Step 2: Create Tauri Rust manifest**

Write `src-tauri/Cargo.toml`:

```toml
[package]
name = "envpilot-tauri"
edition.workspace = true
license.workspace = true
version.workspace = true

[lib]
name = "envpilot_tauri"
crate-type = ["staticlib", "cdylib", "rlib"]

[[bin]]
name = "envpilot-tauri"
path = "src/main.rs"

[dependencies]
envpilot-core = { path = "../crates/envpilot-core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tauri = { version = "2", features = [] }
```

- [ ] **Step 3: Implement Tauri scan command**

Write `src-tauri/src/lib.rs`:

```rust
use envpilot_core::{scan_inventory, ScanOptions};

#[tauri::command]
pub fn scan_inventory_command() -> envpilot_core::model::InventoryReport {
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let workspace_dir = std::env::current_dir().ok();
    scan_inventory(ScanOptions {
        home_dir,
        workspace_dir,
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_inventory_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Write `src-tauri/src/main.rs`:

```rust
fn main() {
    envpilot_tauri::run();
}
```

- [ ] **Step 4: Create frontend package**

Write `ui/package.json`:

```json
{
  "name": "envpilot-ui",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@vitejs/plugin-react": "^5.0.0",
    "vite": "^7.0.0",
    "typescript": "^5.0.0",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "lucide-react": "^0.468.0"
  },
  "devDependencies": {}
}
```

- [ ] **Step 5: Create frontend type mirror**

Write `ui/src/types.ts`:

```ts
export type ToolStatus = "installed" | "partially_detected" | "not_found" | "error";

export interface InventoryReport {
  schema_version: string;
  generated_at: string;
  platform: PlatformInfo;
  tools: AiToolRecord[];
  warnings: Diagnostic[];
}

export interface PlatformInfo {
  os: string;
  arch: string;
  hostname?: string | null;
}

export interface AiToolRecord {
  id: string;
  name: string;
  status: ToolStatus;
  version?: string | null;
  executables: string[];
  install_sources: string[];
  config_files: ConfigFileRecord[];
  config_dirs: string[];
  mcp_servers: unknown[];
  skills: ResourceRecord[];
  agents: ResourceRecord[];
  instructions: ResourceRecord[];
  warnings: Diagnostic[];
  evidence: Evidence[];
}

export interface ConfigFileRecord {
  path: string;
  tool_id: string;
  file_type: string;
  exists: boolean;
  parse_status: string;
  summary?: unknown;
  redacted_preview?: string | null;
  warnings: Diagnostic[];
}

export interface ResourceRecord {
  name: string;
  path: string;
  source: string;
}

export interface Diagnostic {
  severity: string;
  code: string;
  message: string;
  tool_id?: string | null;
  path?: string | null;
}

export interface Evidence {
  kind: string;
  summary: string;
  value?: string | null;
}
```

- [ ] **Step 6: Create Tauri API wrapper**

Write `ui/src/api.ts`:

```ts
import { invoke } from "@tauri-apps/api/core";
import type { InventoryReport } from "./types";

export async function scanInventory(): Promise<InventoryReport> {
  return invoke<InventoryReport>("scan_inventory_command");
}
```

- [ ] **Step 7: Create read-only React shell**

Write `ui/src/main.tsx`:

```tsx
import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

Write `ui/src/App.tsx`:

```tsx
import { useEffect, useState } from "react";
import { scanInventory } from "./api";
import type { InventoryReport } from "./types";
import { Dashboard } from "./pages/Dashboard";
import { Warnings } from "./pages/Warnings";

export function App() {
  const [report, setReport] = useState<InventoryReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    scanInventory().then(setReport).catch((err) => setError(String(err)));
  }, []);

  if (error) {
    return <main><h1>EnvPilot</h1><p>{error}</p></main>;
  }

  if (!report) {
    return <main><h1>EnvPilot</h1><p>Scanning...</p></main>;
  }

  return (
    <main>
      <h1>EnvPilot</h1>
      <Dashboard report={report} />
      <Warnings warnings={report.warnings} />
    </main>
  );
}
```

Write `ui/src/pages/Dashboard.tsx`:

```tsx
import type { InventoryReport } from "../types";

export function Dashboard({ report }: { report: InventoryReport }) {
  return (
    <section>
      <h2>Dashboard</h2>
      <div>
        {report.tools.map((tool) => (
          <article key={tool.id}>
            <h3>{tool.name}</h3>
            <p>Status: {tool.status}</p>
            <p>Version: {tool.version ?? "-"}</p>
            <p>Warnings: {tool.warnings.length}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
```

Write `ui/src/pages/Warnings.tsx`:

```tsx
import type { Diagnostic } from "../types";

export function Warnings({ warnings }: { warnings: Diagnostic[] }) {
  return (
    <section>
      <h2>Warnings</h2>
      {warnings.length === 0 ? (
        <p>No global warnings.</p>
      ) : (
        <ul>
          {warnings.map((warning, index) => (
            <li key={`${warning.code}-${index}`}>
              {warning.severity}: {warning.message}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
```

Write placeholder route components that render static page headings:

`ui/src/pages/ToolDetail.tsx`:

```tsx
export function ToolDetail() {
  return <section><h2>Tool Detail</h2></section>;
}
```

`ui/src/pages/ConfigViewer.tsx`:

```tsx
export function ConfigViewer() {
  return <section><h2>Config Viewer</h2></section>;
}
```

`ui/src/pages/Settings.tsx`:

```tsx
export function Settings() {
  return <section><h2>Settings</h2></section>;
}
```

- [ ] **Step 8: Run Rust check**

Run:

```bash
cargo check --workspace
```

Expected: Rust workspace compiles. If Tauri config files are required by `tauri::generate_context!`, add the minimum Tauri config generated by the official Tauri CLI, then rerun.

- [ ] **Step 9: Commit**

Run:

```bash
git add Cargo.toml src-tauri ui
git commit -m "feat: scaffold read-only desktop console"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 10: Add Project Docs And Verification Commands

**Files:**

- Create: `README.md`
- Create: `AGENTS.md`

- [ ] **Step 1: Write README**

Write `README.md`:

```markdown
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
```

- [ ] **Step 2: Write project AGENTS.md**

Write `AGENTS.md`:

```markdown
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
```

- [ ] **Step 3: Commit**

Run:

```bash
git add README.md AGENTS.md
git commit -m "docs: add envpilot project guidance"
```

Expected: commit succeeds if the project has been initialized as a Git repository.

## Task 11: Full Verification

**Files:**

- No new files.

- [ ] **Step 1: Run formatter**

Run:

```bash
cargo fmt --all -- --check
```

Expected: passes. If it fails, run `cargo fmt --all`, inspect the diff, and rerun the check.

- [ ] **Step 2: Run Rust tests**

Run:

```bash
cargo test --workspace
```

Expected: all Rust tests pass.

- [ ] **Step 3: Run Rust check**

Run:

```bash
cargo check --workspace
```

Expected: all workspace crates compile.

- [ ] **Step 4: Run CLI smoke commands**

Run:

```bash
cargo run -p envpilot -- scan
cargo run -p envpilot -- scan --format json
cargo run -p envpilot -- doctor
cargo run -p envpilot -- config list
```

Expected: commands exit successfully. JSON output includes `schema_version` and `tools`.

- [ ] **Step 5: Run frontend build if dependencies are installed**

Run:

```bash
cd ui && npm run build
```

Expected: frontend builds. If dependencies are not installed, run the project package-manager install command once it is selected and committed.

- [ ] **Step 6: Inspect diff**

Run:

```bash
git status --short
git diff --stat
git diff --check
```

Expected: no whitespace errors; changed files match the MVP scope only.

## Self-Review Notes

- Spec coverage: plan covers scan core, data model, redaction, config analysis, detector contract, initial detectors, CLI, Tauri shell, docs, and verification.
- MVP boundary preserved: no install, uninstall, upgrade, version switching, config editing, raw secret display, cloud sync, or command execution.
- Known implementation risk: exact config paths and schemas for Codex, Claude Code, Gemini CLI, and OpenCode require verification before constants are finalized.
- Current project state risk: the directory was not a Git repository when the plan was written, so commit steps require `git init` or an existing repository before execution.
