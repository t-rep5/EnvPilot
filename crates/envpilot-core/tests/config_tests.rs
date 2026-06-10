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
