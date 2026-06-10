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
                record.evidence.push(path_evidence(
                    path.display().to_string(),
                    "found workspace CLAUDE.md",
                ));
            }
        }
        if !record.instructions.is_empty() {
            record.status = ToolStatus::PartiallyDetected;
        }
        record
    }
}
