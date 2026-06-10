use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use envpilot_core::config::analyze_config_text;
use envpilot_core::model::ToolId;
use envpilot_core::{scan_inventory, ScanOptions};

#[derive(Debug, Parser)]
#[command(name = "envpilot")]
#[command(about = "Development environment inventory for AI tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    Doctor,
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    List,
    Inspect {
        tool_id: String,
        #[arg(long)]
        file: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { format } => run_scan(format),
        Command::Doctor => run_doctor(),
        Command::Config { command } => run_config(command),
    }
}

fn run_scan(format: OutputFormat) -> Result<()> {
    let report = current_report();
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Table => {
            println!("Tool\tStatus\tVersion\tConfigs\tWarnings");
            for tool in report.tools {
                println!(
                    "{}\t{:?}\t{}\t{}\t{}",
                    tool.name,
                    tool.status,
                    tool.version.unwrap_or_else(|| "-".to_string()),
                    tool.config_files.len(),
                    tool.warnings.len()
                );
            }
        }
    }
    Ok(())
}

fn run_doctor() -> Result<()> {
    let report = current_report();
    println!("EnvPilot Doctor");
    for warning in report.warnings {
        println!(
            "{:?}\t{}\t{}",
            warning.severity, warning.code, warning.message
        );
    }
    Ok(())
}

fn run_config(command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::List => {
            let report = current_report();
            println!("Config Files");
            for tool in report.tools {
                for config in tool.config_files {
                    println!("{}\t{}\t{:?}", tool.name, config.path, config.parse_status);
                }
            }
        }
        ConfigCommand::Inspect {
            tool_id,
            file,
            format,
        } => {
            let parsed_tool_id = parse_tool_id(&tool_id)?;
            if let Some(file) = file {
                let text = std::fs::read_to_string(&file)?;
                let record = analyze_config_text(parsed_tool_id, &file, &text)
                    .map_err(anyhow::Error::msg)?;
                match format {
                    OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&record)?),
                    OutputFormat::Table => {
                        println!("Config Inspect");
                        println!("tool_id={tool_id}");
                        println!("file={}", record.path);
                        println!("type={:?}", record.file_type);
                        println!("parse_status={}", record.parse_status);
                        println!("warnings={}", record.warnings.len());
                        if let Some(summary) = record.summary {
                            println!("mcp_servers={}", summary.mcp_server_count);
                            println!(
                                "sensitive_fields={}",
                                summary.sensitive_field_names.join(",")
                            );
                        }
                        if let Some(preview) = record.redacted_preview {
                            println!();
                            println!("{preview}");
                        }
                    }
                }
            } else {
                println!("Config Inspect");
                println!("tool_id={tool_id}");
                println!("file=-");
                println!("format={format:?}");
            }
        }
    }
    Ok(())
}

fn parse_tool_id(value: &str) -> Result<ToolId> {
    match value {
        "codex" => Ok(ToolId::Codex),
        "claude-code" => Ok(ToolId::ClaudeCode),
        "gemini-cli" => Ok(ToolId::GeminiCli),
        "opencode" => Ok(ToolId::Opencode),
        other => anyhow::bail!("unsupported tool id: {other}"),
    }
}

fn current_report() -> envpilot_core::model::InventoryReport {
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
