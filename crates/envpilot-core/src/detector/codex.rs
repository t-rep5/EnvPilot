use super::generic::{empty_record, path_evidence};
use super::{Detector, DetectorContext};
use crate::model::{ResourceRecord, ToolId, ToolStatus};

pub struct CodexDetector;

impl Detector for CodexDetector {
    fn id(&self) -> ToolId {
        ToolId::Codex
    }

    fn detect(&self, context: &DetectorContext) -> crate::model::AiToolRecord {
        let mut record = empty_record(ToolId::Codex, "Codex");
        if let Some(path) = context.resolver.workspace_path("AGENTS.md") {
            if path.exists() {
                record.instructions.push(ResourceRecord {
                    name: "AGENTS.md".to_string(),
                    path: path.display().to_string(),
                    source: "workspace".to_string(),
                });
                record.evidence.push(path_evidence(
                    path.display().to_string(),
                    "found workspace AGENTS.md",
                ));
            }
        }
        if !record.instructions.is_empty() {
            record.status = ToolStatus::PartiallyDetected;
        }
        record
    }
}
