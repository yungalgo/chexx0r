/// Main entry point for chexx0r CLI tool

mod config;
mod domain;
mod social;
mod utils;

use clap::Parser;
use colored::*;
use anyhow::Result;

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

    println!("\n{}", format!("🔍 Checking availability for: {}", args.username).bold());
    println!("{}", "=".repeat(60));

    // Domain checks
    if !args.skip_domains {
        println!("\n{}", "📡 DOMAIN AVAILABILITY".bold().blue());
        
        match domain::check_domains(&args.username, &args.preset, args.tlds.as_deref()).await {
            Ok(_) => {},
            Err(e) => eprintln!("{}", format!("Domain check error: {}", e).red()),
        }
    }

    // Social media checks
    if !args.skip_social {
        println!("\n{}", "📱 SOCIAL MEDIA AVAILABILITY".bold().magenta());
        
        social::check_social_media(&args.username, args.debug).await?;
    }

    println!("\n{}", "=".repeat(60));
    println!("{}", "✅ Check complete!\n".bold().green());

    Ok(())
}
