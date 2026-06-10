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
    if let Some(index) = line.find('=') {
        let key = line[..index].trim();
        if is_sensitive_key(key) {
            return format!("{}= \"{}\"", &line[..index], REDACTED);
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
