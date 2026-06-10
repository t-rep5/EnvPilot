use envpilot_core::{scan_inventory, ScanOptions};

#[tauri::command]
pub fn scan_inventory_command() -> envpilot_core::model::InventoryReport {
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let workspace_dir = std::env::current_dir().ok();
    scan_inventory(ScanOptions {
        home_dir,
        workspace_dir,
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_inventory_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
