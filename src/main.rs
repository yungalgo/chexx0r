/// Main entry point for chexx0r CLI tool

mod config;
mod domain;
mod social;
mod utils;
mod ui;

use clap::Parser;
use anyhow::Result;
use ui::{Dividers, AsciiArtSelector};

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

    // Create ASCII art selector for this run
    let art = AsciiArtSelector::new();
    
    // Ethereal header with prominent ASCII art
    println!();
    println!("{}", art.header);
    println!();
    
    // Use random stylized box pattern from gist
    let (box_top, left_char, right_char, box_bottom, box_width) = Dividers::box_pattern();
    
    let username_display = format!("checking: {}", args.username);
    let content_width = username_display.chars().count();
    
    // Calculate padding to center content
    let total_padding = box_width.saturating_sub(content_width + 2); // +2 for left/right chars
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    
    println!("{}", box_top);
    println!("{}{}{}{}", 
        left_char,
        " ".repeat(left_padding),
        username_display,
        " ".repeat(right_padding) + &right_char.to_string());
    println!("{}", box_bottom);
    println!();
    println!("{}", art.section);
    println!();

    // Domain checks
    if !args.skip_domains {
        domain::check_domains(&args.username, &args.preset, args.tlds.as_deref()).await?;
    }

    // Social media checks
    if !args.skip_social {
        social::check_social_media(&args.username, args.debug).await?;
    }

    // Complete section with stylized box
    println!();
    let (complete_top, complete_left, complete_right, complete_bottom, _) = Dividers::box_pattern();
    let complete_text = Dividers::decorate_text("complete");
    println!("{}", complete_top);
    println!("{}{}{}", complete_left, complete_text, complete_right);
    println!("{}", complete_bottom);
    println!();
    println!("{}", art.header);
    println!();

    Ok(())
}
