use crate::model::{AiToolRecord, Evidence, ToolId, ToolStatus};

pub fn empty_record(id: ToolId, name: &str) -> AiToolRecord {
    AiToolRecord {
        id,
        name: name.to_string(),
        status: ToolStatus::NotFound,
        version: None,
        executables: vec![],
        install_sources: vec![],
        config_files: vec![],
        config_dirs: vec![],
        mcp_servers: vec![],
        skills: vec![],
        agents: vec![],
        instructions: vec![],
        warnings: vec![],
        evidence: vec![],
    }
}

pub fn path_evidence(path: impl Into<String>, summary: &str) -> Evidence {
    Evidence {
        kind: "path".to_string(),
        summary: summary.to_string(),
        value: Some(path.into()),
    }
}
