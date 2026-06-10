use crate::detector::{default_detectors, DetectorContext};
use crate::model::{Diagnostic, InventoryReport, PlatformInfo};
use crate::path::PathResolver;
use chrono::Utc;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub home_dir: PathBuf,
    pub workspace_dir: Option<PathBuf>,
}

pub fn scan_inventory(options: ScanOptions) -> InventoryReport {
    let resolver = PathResolver::new(options.home_dir, options.workspace_dir);
    let context = DetectorContext { resolver };
    let tools = default_detectors()
        .into_iter()
        .map(|detector| detector.detect(&context))
        .collect();

    InventoryReport {
        schema_version: "1".to_string(),
        generated_at: Utc::now().to_rfc3339(),
        platform: PlatformInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: None,
        },
        tools,
        warnings: Vec::<Diagnostic>::new(),
    }
}
