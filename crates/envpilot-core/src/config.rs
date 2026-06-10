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
