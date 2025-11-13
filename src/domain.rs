/// Domain availability checking functionality

use anyhow::Result;
use domain_check_lib::DomainChecker;
use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, presets::UTF8_FULL, Cell};
use std::time::Duration;
use crate::config::get_preset_tlds;

/// Check domain availability for a username across multiple TLDs
pub async fn check_domains(username: &str, preset: &str, custom_tlds: Option<&str>) -> Result<()> {
    let tlds = if let Some(tlds_str) = custom_tlds {
        tlds_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        get_preset_tlds(preset)
    };

    let checker = DomainChecker::new();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Domain", "Status"]);

    let pb = ProgressBar::new(tlds.len() as u64);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} Checking domains... {pos}/{len}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    for tld in &tlds {
        let domain = format!("{}.{}", username, tld);
        
        match checker.check_domain(&domain).await {
            Ok(result) => {
                let status_cell = match result.available {
                    Some(true) => Cell::new("✓ AVAILABLE").fg(comfy_table::Color::Green),
                    Some(false) => Cell::new("✗ TAKEN").fg(comfy_table::Color::Red),
                    None => Cell::new("? CAN'T CHECK").fg(comfy_table::Color::Yellow),
                };
                table.add_row(vec![Cell::new(domain), status_cell]);
            }
            Err(_) => {
                table.add_row(vec![
                    Cell::new(domain),
                    Cell::new("? CAN'T CHECK").fg(comfy_table::Color::Yellow)
                ]);
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();
    println!("{}", table);

    Ok(())
}

