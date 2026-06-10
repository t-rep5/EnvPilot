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
