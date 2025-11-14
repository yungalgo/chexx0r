/// Main entry point for chexx0r CLI tool - orchestrates UI and logic

mod config;
mod domain;
mod social;
mod utils;
mod ui;

use clap::Parser;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, Cell};
use std::time::Duration;
use ui::{Dividers, Colors, render_box, spinner_template, spinner_frames};

#[derive(Parser, Debug)]
#[command(name = "chexx0r")]
#[command(author = "yungalgo")]
#[command(version = "0.1.0")]
#[command(about = "Check domain and social media username availability", long_about = None)]
struct Args {
    /// Username to check
    #[arg(value_name = "USERNAME")]
    username: String,

    /// TLD preset to use (startup, enterprise, country, or custom TLDs)
    #[arg(short, long, default_value = "startup")]
    preset: String,

    /// Custom TLDs (comma-separated, e.g., com,org,io)
    #[arg(short, long)]
    tlds: Option<String>,

    /// Skip domain checks
    #[arg(long)]
    skip_domains: bool,

    /// Skip social media checks
    #[arg(long)]
    skip_social: bool,

    /// Show debug output for social media checks
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Select a single box pattern for all boxes in this run
    let box_pattern = Dividers::box_pattern();
    
    println!();
    
    // Show initial checking box
    let initial_text = format!("checking: {}", args.username);
    let colored_text = Colors::checking(&initial_text).to_string();
    let (box_top, left_char, right_char, box_bottom) = Dividers::create_box_with_pattern(&colored_text, Some(box_pattern));
    
    println!("{}", box_top);
    let box_width = Dividers::strip_ansi_codes(&box_top).chars().count().max(
        Dividers::strip_ansi_codes(&box_bottom).chars().count()
    );
    
    let content_width = Dividers::strip_ansi_codes(&colored_text).chars().count();
    let total_padding = box_width.saturating_sub(content_width + 2);
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    println!("{}{}{}{}", 
        left_char,
        " ".repeat(left_padding),
        colored_text,
        " ".repeat(right_padding) + &right_char.to_string());
    println!("{}", box_bottom);
    
    // Create a single spinner that persists across all checks
    let frames = spinner_frames();
    let frame_refs: Vec<&str> = frames.iter().map(|s| s.as_str()).collect();
    let pb = ProgressBar::new(100); // Dummy total, we'll manage it manually
    pb.set_style(
        ProgressStyle::with_template(&spinner_template())
            .unwrap()
            .tick_strings(&frame_refs),
    );
    pb.enable_steady_tick(Duration::from_millis(150));
    
    // Domain checks
    let domain_results = if !args.skip_domains {
        pb.set_message("scanning domains".to_string());
        let results = domain::check_domains(&args.username, &args.preset, args.tlds.as_deref()).await?;
        Some(results)
    } else {
        None
    };
    
    // Social media checks
    let social_results = if !args.skip_social {
        pb.set_message("scanning socials".to_string());
        let results = social::check_social_media(&args.username, args.debug).await?;
        Some(results)
    } else {
        None
    };
    
    // Clear checking box and spinner before showing results
    print!("\x1b[4A\x1b[0J"); // Clear checking box (3 lines) + spinner (1 line)
    pb.finish_and_clear();
    
    // Render domain results
    if let Some(results) = domain_results {
        println!();
        render_box("domains", box_pattern);
        println!();
        
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::NOTHING);
        table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        
        for result in results {
            let status_cell = match result.available {
                Some(true) => Cell::new("available").fg(comfy_table::Color::Green),
                Some(false) => Cell::new("taken").fg(comfy_table::Color::White),
                None => Cell::new("unknown").fg(comfy_table::Color::Grey),
            };
            table.add_row(vec![Cell::new(result.domain), status_cell]);
        }
        
        // Render table in box
        let table_str = format!("{}", table);
        let table_width = table_str.lines()
            .map(|l| Dividers::strip_ansi_codes(l).chars().count())
            .max()
            .unwrap_or(50);
        let (box_top, box_left, box_right, box_bottom) = Dividers::create_box_with_pattern(&" ".repeat(table_width), Some(box_pattern));
        println!("{}", box_top);
        
        for line in table_str.lines() {
            let line_width = Dividers::strip_ansi_codes(line).chars().count();
            let padding = table_width.saturating_sub(line_width);
            println!("{}{}{}{}", box_left, line, " ".repeat(padding), box_right);
        }
        
        println!("{}", box_bottom);
    }
    
    // Render social results
    if let Some(results) = social_results {
        println!();
        render_box("social", box_pattern);
        println!();
        
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::NOTHING);
        table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        
        for result in results {
            let status_cell = match result.status {
                social::SocialStatus::Available => Cell::new("available").fg(comfy_table::Color::Green),
                social::SocialStatus::Taken => Cell::new("taken").fg(comfy_table::Color::White),
                social::SocialStatus::Invalid => Cell::new("invalid").fg(comfy_table::Color::White),
                social::SocialStatus::Unknown => Cell::new("unknown").fg(comfy_table::Color::Grey),
            };
            table.add_row(vec![Cell::new(result.platform), status_cell]);
        }
        
        // Render table in box
        let table_str = format!("{}", table);
        let table_width = table_str.lines()
            .map(|l| Dividers::strip_ansi_codes(l).chars().count())
            .max()
            .unwrap_or(50);
        let (box_top, box_left, box_right, box_bottom) = Dividers::create_box_with_pattern(&" ".repeat(table_width), Some(box_pattern));
        println!("{}", box_top);
        
        for line in table_str.lines() {
            let line_width = Dividers::strip_ansi_codes(line).chars().count();
            let padding = table_width.saturating_sub(line_width);
            println!("{}{}{}{}", box_left, line, " ".repeat(padding), box_right);
        }
        
        println!("{}", box_bottom);
    }

    // Complete section
    println!();
    render_box("complete", box_pattern);
    println!();

    Ok(())
}
