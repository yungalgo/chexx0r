/// Social media platform availability checking functionality

use anyhow::{Result, Context};
use reqwest::Client;
use futures::future::join_all;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, presets::UTF8_FULL, Cell};
use colored::*;

use crate::config::SOCIAL_PLATFORMS;
use crate::utils::{validate_instagram_username, validate_youtube_username, validate_tiktok_username};

/// Check social media platform availability for a username
pub async fn check_social_media(username: &str, debug: bool) -> Result<()> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;

    let pb = ProgressBar::new(SOCIAL_PLATFORMS.len() as u64);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.magenta} Checking social platforms... {pos}/{len}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Platform", "Status"]);

    let pb_clone = pb.clone();
    let futures = SOCIAL_PLATFORMS.iter().map(|platform| {
        let client = client.clone();
        let username = username.to_string();
        let platform_name = platform.name;
        let url = platform.url_template.replace("{}", &username);
        let pb = pb_clone.clone();

        async move {
            // Validate username format first
            let validation_result = match platform_name {
                "Instagram" => validate_instagram_username(&username),
                "YouTube" => validate_youtube_username(&username),
                "TikTok" => validate_tiktok_username(&username),
                _ => Ok(()), // Unknown platform, skip validation
            };
            
            let result = match validation_result {
                Ok(_) => {
                    // Username is valid, proceed with HTTP check
                    check_social_platform(&client, &url, platform_name, debug).await
                }
                Err(_) => {
                    // Username is invalid, return error immediately
                    Err(anyhow::anyhow!("Invalid username format"))
                }
            };
            
            pb.inc(1);
            (platform_name, result)
        }
    });

    let results = join_all(futures).await;
    pb.finish_and_clear();

    for (platform_name, result) in results {
        match result {
            Ok(is_taken) => {
                let status_cell = if is_taken {
                    Cell::new("✗ TAKEN").fg(comfy_table::Color::Red)
                } else {
                    Cell::new("✓ AVAILABLE").fg(comfy_table::Color::Green)
                };
                table.add_row(vec![Cell::new(platform_name), status_cell]);
            }
            Err(e) => {
                // Check if it's a validation error or network error
                let error_msg = e.to_string();
                let status = if error_msg.contains("Invalid username format") {
                    "✗ INVALID FORMAT"
                } else {
                    "? CAN'T CHECK"
                };
                
                let status_cell = if status == "✗ INVALID FORMAT" {
                    Cell::new(status).fg(comfy_table::Color::Red)
                } else {
                    Cell::new(status).fg(comfy_table::Color::Yellow)
                };
                
                table.add_row(vec![
                    Cell::new(platform_name),
                    status_cell
                ]);
            }
        }
    }

    println!("{}", table);

    Ok(())
}

/// Check a single social media platform for username availability
async fn check_social_platform(client: &Client, url: &str, platform: &str, debug: bool) -> Result<bool> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    let status = response.status();
    
    // YouTube: Returns 404 for non-existent profiles, 200 for existing ones
    if url.contains("youtube.com") {
        if status.as_u16() == 404 {
            return Ok(false); // Available - channel doesn't exist
        } else if status.is_success() {
            return Ok(true); // Taken - channel exists
        } else {
            // Other status codes - can't determine
            return Err(anyhow::anyhow!("HTTP {}", status.as_u16()));
        }
    }
    
    // Instagram and TikTok return 200 but show specific error messages
    if status.as_u16() == 404 {
        return Ok(false); // Available
    }
    
    if status.is_success() {
        let body = response.text().await?;
        let body_lower = body.to_lowercase();
        
        if debug && platform == "Instagram" {
            print_instagram_debug(&body, &body_lower, url);
        }
        
        // Check for platform-specific patterns
        check_platform_availability(&body_lower, url)
    } else {
        // Other status codes - can't determine
        Err(anyhow::anyhow!("HTTP {}", status.as_u16()))
    }
}

/// Print debug information for Instagram responses
fn print_instagram_debug(body: &str, body_lower: &str, url: &str) {
    let username_from_url = url.split('/').last().unwrap_or("");
    println!("\n{}", "=== DEBUG: Instagram Response ===".yellow());
    println!("Body length: {} bytes", body.len());
    println!("Username from URL: {}", username_from_url);
    
    // Check title patterns
    let title_contains_username = !username_from_url.is_empty() && 
        (body_lower.contains(&format!("<title> (@{}", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("<title> (&#064;{}", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("(@{})", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("(&#064;{})", username_from_url).to_lowercase()));
    let is_generic_title = body_lower.contains("<title>instagram</title>") ||
        body_lower.contains("<title>login • instagram</title>");
    
    println!("Title contains username: {}", title_contains_username);
    println!("Is generic title: {}", is_generic_title);
    println!("Contains 'profilepage': {}", body_lower.contains("profilepage"));
    
    // Extract title for display
    if let Some(title_start) = body_lower.find("<title>") {
        if let Some(title_end) = body_lower[title_start..].find("</title>") {
            let title = &body_lower[title_start..title_start + title_end + 8];
            println!("Title tag: {}", title);
        }
    }
    
    // Save to file for inspection
    use std::fs;
    let filename = format!("/tmp/instagram_debug_{}.html", username_from_url);
    if let Err(e) = fs::write(&filename, body) {
        println!("Failed to write debug file: {}", e);
    } else {
        println!("Full response saved to: {}", filename);
    }
    
    println!("{}", "================================\n".yellow());
}

/// Check platform-specific availability patterns in HTML body
fn check_platform_availability(body_lower: &str, url: &str) -> Result<bool> {
    // Instagram: Check title tag to determine if profile exists
    if url.contains("instagram.com") {
        return check_instagram_availability(body_lower, url);
    }
    
    // TikTok: Existing profiles have "uniqueId" in embedded JSON
    if url.contains("tiktok.com") {
        return check_tiktok_availability(body_lower);
    }
    
    // For other platforms or if no specific pattern matched, assume profile exists
    Ok(true) // Taken
}

/// Check Instagram profile availability based on HTML content
#[cfg_attr(test, allow(dead_code))]
pub fn check_instagram_availability(body_lower: &str, url: &str) -> Result<bool> {
    // Extract username from URL
    let username_from_url = url.split('/').last().unwrap_or("");
    
    // Real profiles have title like: "<title> (@username) • Instagram photos and videos</title>"
    // Fake/non-existent profiles have title: "<title>Instagram</title>" or "<title>Login • Instagram</title>"
    
    // Check if title contains the username (most reliable indicator)
    // Instagram uses HTML entity &#064; for @ in titles, or regular @
    // Format: "<title> (@username) • Instagram" or "<title> (&#064;username) • Instagram"
    let title_contains_username = !username_from_url.is_empty() && 
        (body_lower.contains(&format!("<title> (@{}", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("<title> (&#064;{}", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("(@{})", username_from_url).to_lowercase()) ||
         body_lower.contains(&format!("(&#064;{})", username_from_url).to_lowercase()));
    
    // Check for generic Instagram title (indicates profile doesn't exist)
    let is_generic_title = body_lower.contains("<title>instagram</title>") ||
        body_lower.contains("<title>login • instagram</title>");
    
    if title_contains_username {
        Ok(true) // Taken - username found in title
    } else if is_generic_title {
        Ok(false) // Available - generic Instagram page
    } else {
        // Fallback: if we can't determine from title, check for profilepage
        // (but this is less reliable as it appears on both)
        if body_lower.contains("profilepage") {
            Ok(true) // Likely taken
        } else {
            Ok(false) // Likely available
        }
    }
}

/// Check TikTok profile availability based on HTML content
#[cfg_attr(test, allow(dead_code))]
pub fn check_tiktok_availability(body_lower: &str) -> Result<bool> {
    // TikTok: Both real and fake profiles return HTTP 200
    // Real profiles have:
    //   - "uniqueId":"username" in embedded JSON
    //   - "statusCode":0
    // Non-existent profiles have:
    //   - "statusCode":10221 (or other non-zero codes)
    //   - "statusMsg":"user banned" or similar error messages
    //   - No uniqueId field
    
    // Check for positive indicators first
    let has_unique_id = body_lower.contains("\"uniqueid\":\"");
    let has_status_code_zero = body_lower.contains("\"statuscode\":0");
    
    // Check for negative indicators
    let has_error_status_code = body_lower.contains("\"statuscode\":10221") ||
                                body_lower.contains("\"statuscode\":10222") ||
                                body_lower.contains("\"statusmsg\":\"user banned\"") ||
                                body_lower.contains("\"statusmsg\":\"user not found\"");
    
    if has_unique_id && has_status_code_zero {
        Ok(true) // Taken - profile exists
    } else if has_error_status_code {
        Ok(false) // Available - explicit error status
    } else if has_unique_id {
        // Has uniqueId but no statusCode check - assume taken (fallback)
        Ok(true)
    } else {
        // No uniqueId and no error status - assume available
        Ok(false)
    }
}

