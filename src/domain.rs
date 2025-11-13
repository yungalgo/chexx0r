/// Domain availability checking functionality

use anyhow::Result;
use domain_check_lib::DomainChecker;
use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, presets::UTF8_FULL, Cell};
use std::time::Duration;
use crate::config::get_preset_tlds;
use crate::ui::{Colors, spinner_template, spinner_frames, AsciiArtSelector, Dividers};

/// Check domain availability for a username across multiple TLDs
pub async fn check_domains(username: &str, preset: &str, custom_tlds: Option<&str>) -> Result<()> {
    let tlds = if let Some(tlds_str) = custom_tlds {
        tlds_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        get_preset_tlds(preset)
    };

    // Create ASCII art for this section
    let art = AsciiArtSelector::new();
    
    // Section header with decorative text
    println!("{}", art.section);
    println!("{}", Colors::section(&format!("  {}", Dividers::decorate_text("domains"))));
    println!("{}", art.section);

    let checker = DomainChecker::new();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["domain", "status"]);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    let pb = ProgressBar::new(tlds.len() as u64);
    let frames = spinner_frames();
    let frame_refs: Vec<&str> = frames.iter().map(|s| s.as_str()).collect();
    pb.set_style(
        ProgressStyle::with_template(&spinner_template())
            .unwrap()
            .tick_strings(&frame_refs),
    );
    pb.set_message("scanning".to_string());
    pb.enable_steady_tick(Duration::from_millis(150));

    for tld in &tlds {
        let domain = format!("{}.{}", username, tld);
        
        match checker.check_domain(&domain).await {
            Ok(result) => {
                let status_cell = match result.available {
                    Some(true) => Cell::new("available").fg(comfy_table::Color::Green),
                    Some(false) => Cell::new("taken").fg(comfy_table::Color::White),
                    None => Cell::new("unknown").fg(comfy_table::Color::Grey),
                };
                table.add_row(vec![Cell::new(domain), status_cell]);
            }
            Err(_) => {
                table.add_row(vec![
                    Cell::new(domain),
                    Cell::new("unknown").fg(comfy_table::Color::Grey)
                ]);
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();
    println!();
    
    // Decorative box around table
    let (box_top, _, _, box_bottom, _) = Dividers::box_pattern();
    println!("{}", box_top);
    println!("{}", table);
    println!("{}", box_bottom);

    Ok(())
}

