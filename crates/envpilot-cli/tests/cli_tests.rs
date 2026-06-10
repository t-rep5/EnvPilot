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

#[test]
fn config_inspect_redacts_file_content() {
    let temp = tempfile::tempdir().expect("tempdir");
    let config = temp.path().join("config.json");
    std::fs::write(
        &config,
        r#"{"mcpServers":{"demo":{"command":"demo","env":{"API_KEY":"secret"}}}}"#,
    )
    .expect("write config");

    let mut cmd = Command::cargo_bin("envpilot").expect("binary");
    cmd.args([
        "config",
        "inspect",
        "codex",
        "--file",
        config.to_str().expect("utf8 path"),
    ])
    .assert()
    .success()
    .stdout(contains("[REDACTED]"))
    .stdout(contains("mcp_servers=1"));
}
