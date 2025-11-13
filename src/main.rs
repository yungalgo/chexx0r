use clap::Parser;
use colored::*;
use anyhow::{Result, Context};
use domain_check_lib::DomainChecker;
use futures::future::join_all;
use std::time::Duration;

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

#[derive(Debug)]
struct SocialPlatform {
    name: &'static str,
    url_template: &'static str,
    color: Color,
}

const SOCIAL_PLATFORMS: &[SocialPlatform] = &[
    SocialPlatform {
        name: "YouTube",
        url_template: "https://www.youtube.com/@{}",
        color: Color::Red,
    },
    SocialPlatform {
        name: "Instagram",
        url_template: "https://www.instagram.com/{}",
        color: Color::Magenta,
    },
    SocialPlatform {
        name: "TikTok",
        url_template: "https://www.tiktok.com/@{}",
        color: Color::Cyan,
    },
];

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("\n{}", format!("🔍 Checking availability for: {}", args.username).bold());
    println!("{}", "=".repeat(60));

    // Domain checks
    if !args.skip_domains {
        println!("\n{}", "📡 DOMAIN AVAILABILITY".bold().blue());
        println!("{}", "-".repeat(60));
        
        match check_domains(&args.username, &args.preset, args.tlds.as_deref()).await {
            Ok(_) => {},
            Err(e) => eprintln!("{}", format!("Domain check error: {}", e).red()),
        }
    }

    // Social media checks
    if !args.skip_social {
        println!("\n{}", "📱 SOCIAL MEDIA AVAILABILITY".bold().magenta());
        println!("{}", "-".repeat(60));
        
        check_social_media(&args.username, args.debug).await?;
    }

    println!("\n{}", "=".repeat(60));
    println!("{}", "✅ Check complete!\n".bold().green());

    Ok(())
}

async fn check_domains(username: &str, preset: &str, custom_tlds: Option<&str>) -> Result<()> {
    let tlds = if let Some(tlds_str) = custom_tlds {
        tlds_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        get_preset_tlds(preset)
    };

    let checker = DomainChecker::new();

    for tld in tlds {
        let domain = format!("{}.{}", username, tld);
        
        match checker.check_domain(&domain).await {
            Ok(result) => {
                let status = match result.available {
                    Some(true) => "AVAILABLE".green().bold(),
                    Some(false) => "TAKEN".red().bold(),
                    None => "CAN'T CHECK".yellow().bold(),
                };
                println!("{:<30} {}", domain, status);
            }
            Err(_) => {
                println!("{:<30} {}", domain, "CAN'T CHECK".yellow().bold());
            }
        }
    }

    Ok(())
}

async fn check_social_media(username: &str, debug: bool) -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;

    let futures = SOCIAL_PLATFORMS.iter().map(|platform| {
        let client = client.clone();
        let username = username.to_string();
        let platform_name = platform.name;
        let url = platform.url_template.replace("{}", &username);
        let color = platform.color;

        async move {
            match check_social_platform(&client, &url, platform_name, debug).await {
                Ok(is_taken) => {
                    let status = if is_taken {
                        "TAKEN".red().bold()
                    } else {
                        "AVAILABLE".green().bold()
                    };
                    println!("{:<30} {}", 
                        platform_name.color(color), 
                        status
                    );
                }
                Err(_) => {
                    println!("{:<30} {}", 
                        platform_name.color(color), 
                        "CAN'T CHECK".yellow().bold()
                    );
                }
            }
        }
    });

    join_all(futures).await;

    Ok(())
}

async fn check_social_platform(client: &reqwest::Client, url: &str, platform: &str, debug: bool) -> Result<bool> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    let status = response.status();
    
    // YouTube returns 404 for non-existent profiles
    if status.as_u16() == 404 {
        return Ok(false); // Available
    }
    
    // Instagram and TikTok return 200 but show specific error messages
    if status.is_success() {
        let body = response.text().await?;
        let body_lower = body.to_lowercase();
        
        if debug && platform == "Instagram" {
            println!("\n{}", "=== DEBUG: Instagram Response ===".yellow());
            println!("Body length: {} bytes", body.len());
            println!("Contains 'profile isn': {}", body_lower.contains("profile isn"));
            println!("Contains 'profilepage': {}", body_lower.contains("profilepage"));
            println!("Contains 'not found': {}", body_lower.contains("not found"));
            
            // Save to file for inspection
            use std::fs;
            let filename = format!("/tmp/instagram_debug_{}.html", url.split('/').last().unwrap_or("unknown"));
            if let Err(e) = fs::write(&filename, &body) {
                println!("Failed to write debug file: {}", e);
            } else {
                println!("Full response saved to: {}", filename);
            }
            
            // Show a snippet of the body
            let snippet = if body.len() > 500 { &body[..500] } else { &body };
            println!("First 500 chars:\n{}", snippet);
            println!("{}", "================================\n".yellow());
        }
        
        // Check for platform-specific patterns
        
        // Instagram: Non-existent profiles have title="Instagram" or "Login • Instagram"
        // Existing profiles have title like "@username • Instagram photos and videos"
        if body_lower.contains("<title>instagram</title>") || 
           body_lower.contains("<title>login • instagram</title>") {
            return Ok(false); // Available
        }
        
        // TikTok: Existing profiles have "uniqueId" in embedded JSON
        // Non-existent profiles don't have this field
        if url.contains("tiktok.com") {
            // If uniqueId is present, profile exists (taken)
            // If not present, profile doesn't exist (available)
            if body_lower.contains("\"uniqueid\":\"") {
                return Ok(true); // Taken
            } else {
                return Ok(false); // Available
            }
        }
        
        // For other platforms or if no specific pattern matched, assume profile exists
        return Ok(true); // Taken
    }
    
    // Other status codes - can't determine
    Err(anyhow::anyhow!("HTTP {}", status.as_u16()))
}

fn get_preset_tlds(preset: &str) -> Vec<String> {
    match preset.to_lowercase().as_str() {
        "startup" => vec![
            "com", "org", "io", "ai", "tech", "app", "dev", "xyz"
        ],
        "enterprise" => vec![
            "com", "org", "net", "info", "biz", "us"
        ],
        "country" => vec![
            "us", "uk", "de", "fr", "ca", "au", "jp", "br", "in"
        ],
        _ => vec!["com", "org", "io", "ai", "tech", "app", "dev", "xyz"],
    }
    .iter()
    .map(|s| s.to_string())
    .collect()
}

