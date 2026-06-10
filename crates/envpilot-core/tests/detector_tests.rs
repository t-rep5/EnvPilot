use envpilot_core::detector::{
    ClaudeDetector, CodexDetector, Detector, DetectorContext, GeminiDetector,
};
use envpilot_core::model::{AiToolRecord, ToolId, ToolStatus};
use envpilot_core::path::PathResolver;
use std::fs;
use std::path::PathBuf;

struct FakeDetector;

impl Detector for FakeDetector {
    fn id(&self) -> ToolId {
        ToolId::Codex
    }

    fn detect(&self, _context: &DetectorContext) -> AiToolRecord {
        envpilot_core::detector::generic::empty_record(ToolId::Codex, "Codex")
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
