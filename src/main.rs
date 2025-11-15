/// Main entry point for chexx0r CLI tool
/// 
/// EXPRESS CONCERNS:
/// - CLI argument parsing and validation
/// - Orchestrating domain and social media checks
/// - Coordinating UI rendering (delegates to ui module)
/// - Progress indication during checks
/// 
/// DOES NOT:
/// - Perform domain/social checking logic (delegates to domain/social modules)
/// - Render UI details (delegates to ui module)
/// - Validate usernames (delegates to utils module)

mod config;
mod domain;
mod social;
mod utils;
mod ui;

use clap::Parser;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use colored::Colorize;
use ui::{Dividers, Colors, render_box, spinner_template, spinner_frames, add_decorative_fill, render_domain_results, render_social_results};

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

    println!();
    
    // Show initial checking box - use a random pattern
    let initial_text = format!("checking: {}", args.username);
    let colored_text = Colors::checking(&initial_text).to_string();
    let checking_box_pattern = Dividers::box_pattern();
    let (box_top, left_char, right_char, box_bottom, box_width) = Dividers::create_box_with_pattern(&colored_text, Some(checking_box_pattern));
    
    println!("{}", add_decorative_fill(&box_top));
    let content_width = Dividers::strip_ansi_codes(&colored_text).chars().count();
    let total_padding = box_width.saturating_sub(content_width + 2);
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    let content_line = format!("{}{}{}{}", 
        left_char,
        " ".repeat(left_padding),
        colored_text,
        " ".repeat(right_padding) + &right_char.to_string());
    println!("{}", add_decorative_fill(&content_line));
    println!("{}", add_decorative_fill(&box_bottom));
    
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
    
    // Render domain results - delegate to UI module
    if let Some(results) = domain_results {
        render_domain_results(&results);
    }
    
    // Render social results - delegate to UI module
    if let Some(results) = social_results {
        render_social_results(&results);
    }

    // Complete section
    println!();
    let complete_box_pattern = Dividers::box_pattern();
    let complete_text = "complete".bright_magenta().to_string();
    render_box(&complete_text, complete_box_pattern);
    println!();

    Ok(())
}
