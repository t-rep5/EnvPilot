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
