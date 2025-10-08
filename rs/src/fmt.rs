use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use dprint_plugin_typescript::configuration::Configuration as TypeScriptConfiguration;
use dprint_plugin_typescript::configuration::ConfigurationBuilder as TypeScriptConfigurationBuilder;
use dprint_plugin_typescript::FormatTextOptions;
use dprint_plugin_json::configuration::ConfigurationBuilder as JsonConfigurationBuilder;
use dprint_plugin_json::format_text as format_json_text;
use dprint_plugin_css::configuration::Configuration as CssConfiguration;
use dprint_plugin_css::configuration::ConfigurationBuilder as CssConfigurationBuilder;
use dprint_plugin_css::format_text as format_css_text;

pub fn format_project_programmatically(start_path: &str) -> Result<()> {
    println!("Starting formatting process with filter diagnostics...");

    let walker = WalkDir::new(start_path)
        .into_iter()
        .filter_entry(|entry| {
            let is_skipped = is_skipped_dir(entry);
            println!("[FILTER] Evaluating: {:?}. -> Should Skip? {}", entry.path(), is_skipped);
            !is_skipped
        });

    let mut formatted_count = 0;
    for entry in walker {
        match entry {
            Ok(dir_entry) => {
                if dir_entry.file_type().is_file() {
                    if let Some(path_str) = dir_entry.path().to_str() {
                        if fmt_file_helper(path_str)? {
                            formatted_count += 1;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error walking directory: {}", e),
        }
    }

    println!("Formatting process completed. Formatted {} file(s).", formatted_count);
    Ok(())
}

fn is_skipped_dir(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s == "node_modules" || s == "target" || s.starts_with('.'))
         .unwrap_or(false)
}

fn fmt_file_helper(path_str: &str) -> Result<bool> {
    let file_path = Path::new(path_str);
    let original_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return Ok(false),
    };

    let ext_option = file_path.extension().and_then(|s| s.to_str());
    let ext_str = ext_option.unwrap_or("");

    let formatted_content = match ext_str {
        "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" => {
            let ts_config: TypeScriptConfiguration = TypeScriptConfigurationBuilder::new().build();
            let options = FormatTextOptions {
                path: file_path, text: original_content.clone(), extension: ext_option,
                config: &ts_config, external_formatter: None,
            };
            dprint_plugin_typescript::format_text(options)?
        }
        "json" => {
            let config = JsonConfigurationBuilder::new().build();
            format_json_text(file_path, &original_content, &config)?
        }
        "css" | "scss" => {
            let config: CssConfiguration = CssConfigurationBuilder::new().build();
            Some(format_css_text(file_path, &original_content, &config)?)
        }
        _ => None,
    };

    if let Some(formatted) = formatted_content {
        if formatted != original_content {
            fs::write(file_path, formatted)?;
            println!("Formatted: {}", path_str);
            return Ok(true);
        }
    }
    Ok(false)
}