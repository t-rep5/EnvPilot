export type ToolStatus = "installed" | "partially_detected" | "not_found" | "error";

export interface InventoryReport {
  schema_version: string;
  generated_at: string;
  platform: PlatformInfo;
  tools: AiToolRecord[];
  warnings: Diagnostic[];
}

export interface PlatformInfo {
  os: string;
  arch: string;
  hostname?: string | null;
}

export interface AiToolRecord {
  id: string;
  name: string;
  status: ToolStatus;
  version?: string | null;
  executables: string[];
  install_sources: string[];
  config_files: ConfigFileRecord[];
  config_dirs: string[];
  mcp_servers: unknown[];
  skills: ResourceRecord[];
  agents: ResourceRecord[];
  instructions: ResourceRecord[];
  warnings: Diagnostic[];
  evidence: Evidence[];
}

export interface ConfigFileRecord {
  path: string;
  tool_id: string;
  file_type: string;
  exists: boolean;
  parse_status: string;
  summary?: unknown;
  redacted_preview?: string | null;
  warnings: Diagnostic[];
}

export interface ResourceRecord {
  name: string;
  path: string;
  source: string;
}

export interface Diagnostic {
  severity: string;
  code: string;
  message: string;
  tool_id?: string | null;
  path?: string | null;
}

export interface Evidence {
  kind: string;
  summary: string;
  value?: string | null;
}
