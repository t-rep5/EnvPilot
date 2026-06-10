import { invoke } from "@tauri-apps/api/core";
import type { InventoryReport } from "./types";

export async function scanInventory(): Promise<InventoryReport> {
  return invoke<InventoryReport>("scan_inventory_command");
}
