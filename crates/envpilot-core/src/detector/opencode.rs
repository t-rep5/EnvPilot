use super::generic::empty_record;
use super::{Detector, DetectorContext};
use crate::model::ToolId;

pub struct OpencodeDetector;

impl Detector for OpencodeDetector {
    fn id(&self) -> ToolId {
        ToolId::Opencode
    }

    fn detect(&self, _context: &DetectorContext) -> crate::model::AiToolRecord {
        empty_record(ToolId::Opencode, "OpenCode")
    }
}
