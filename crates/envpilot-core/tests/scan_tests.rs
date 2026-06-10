use envpilot_core::model::{ToolId, ToolStatus};
use envpilot_core::{scan_inventory, ScanOptions};
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
    let codex = report
        .tools
        .iter()
        .find(|tool| tool.id == ToolId::Codex)
        .expect("codex");
    assert_eq!(codex.status, ToolStatus::PartiallyDetected);
}
