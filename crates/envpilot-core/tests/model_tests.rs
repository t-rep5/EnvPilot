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
