/// Social media platform availability checking functionality
/// 
/// EXPRESS CONCERNS:
/// - HTTP requests to social media platforms
/// - Parsing HTML responses to determine availability
/// - Platform-specific detection logic (Instagram, YouTube, TikTok)
/// - Username validation integration
/// 
/// DOES NOT:
/// - Render UI or print to console (except debug mode)
/// - Handle CLI arguments
/// - Manage progress bars or spinners
/// - Format output for display

use anyhow::{Result, Context};
use reqwest::Client;
use futures::future::join_all;
use std::time::Duration;
use crate::config::SOCIAL_PLATFORMS;
use crate::utils::{validate_instagram_username, validate_youtube_username, validate_tiktok_username};

/// Social media check result
pub struct SocialResult {
    pub platform: String,
    pub status: SocialStatus,
}

/// Social media availability status
pub enum SocialStatus {
    Available,
    Taken,
    Invalid,
    Unknown,
}

/// Check social media platform availability for a username
/// Returns a vector of social results - NO UI rendering
pub async fn check_social_media(username: &str, debug: bool) -> Result<Vec<SocialResult>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;

    let futures = SOCIAL_PLATFORMS.iter().map(|platform| {
        let client = client.clone();
        let username = username.to_string();
        let platform_name = platform.name;
        let url = platform.url_template.replace("{}", &username);

        async move {
            // Validate username format first
            let validation_result = match platform_name {
                "instagram" => validate_instagram_username(&username),
                "youtube" => validate_youtube_username(&username),
                "tiktok" => validate_tiktok_username(&username),
                _ => Ok(()), // Unknown platform, skip validation
            };
            
            let result = match validation_result {
                Ok(_) => {
                    // Username is valid, proceed with HTTP check
                    check_social_platform(&client, &url, platform_name, debug).await
                }
                Err(_) => {
                    // Username is invalid, return error immediately
                    Err(anyhow::anyhow!("invalid username format"))
                }
            };
            
            (platform_name, result)
        }
    });

    let results = join_all(futures).await;
    
    let mut social_results = Vec::new();
    for (platform_name, result) in results {
        let status = match result {
            Ok(is_taken) => {
                if is_taken {
                    SocialStatus::Taken
                } else {
                    SocialStatus::Available
                }
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("invalid username format") {
                    SocialStatus::Invalid
                } else {
                    SocialStatus::Unknown
                }
            }
        };
        
        social_results.push(SocialResult {
            platform: platform_name.to_string(),
            status,
        });
    }

    Ok(social_results)
}

/// Check a single social media platform for username availability
async fn check_social_platform(client: &Client, url: &str, _platform: &str, _debug: bool) -> Result<bool> {
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
        
        // Debug output is handled by caller (main.rs) via UI module
        // For now, we just proceed with availability check
        // The caller can access debug info through error context if needed
        
        // Check for platform-specific patterns
        check_platform_availability(&body_lower, url)
    } else {
        // Other status codes - can't determine
        Err(anyhow::anyhow!("HTTP {}", status.as_u16()))
    }
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
