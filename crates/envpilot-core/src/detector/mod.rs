pub mod claude;
pub mod codex;
pub mod gemini;
pub mod generic;
pub mod opencode;

pub use claude::ClaudeDetector;
pub use codex::CodexDetector;
pub use gemini::GeminiDetector;
pub use opencode::OpencodeDetector;

use crate::model::{AiToolRecord, ToolId};
use crate::path::PathResolver;

#[derive(Debug, Clone)]
pub struct DetectorContext {
    pub resolver: PathResolver,
}

pub trait Detector {
    fn id(&self) -> ToolId;
    fn detect(&self, context: &DetectorContext) -> AiToolRecord;
}

pub fn default_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(CodexDetector),
        Box::new(ClaudeDetector),
        Box::new(GeminiDetector),
        Box::new(OpencodeDetector),
    ]
}
