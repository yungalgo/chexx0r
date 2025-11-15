/// Username validation functions for different social media platforms
/// 
/// EXPRESS CONCERNS:
/// - Validating username format against platform-specific rules
/// - Instagram, YouTube, TikTok username validation
/// - Pure validation logic with no side effects
/// 
/// DOES NOT:
/// - Make HTTP requests
/// - Render UI or print output
/// - Check actual availability (only validates format)
/// - Handle errors beyond returning Ok/Err

/// Validates Instagram username format
/// Rules:
/// - 1-30 characters
/// - Letters, numbers, periods, underscores only
/// - Can't start or end with period
/// - Can't have consecutive periods
pub fn validate_instagram_username(username: &str) -> Result<(), ()> {
    if username.is_empty() || username.len() > 30 {
        return Err(());
    }
    
    if username.starts_with('.') || username.ends_with('.') {
        return Err(());
    }
    
    if username.contains("..") {
        return Err(());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_') {
        return Err(());
    }
    
    Ok(())
}

/// Validates YouTube username format
/// Rules:
/// - 3-20 characters
/// - Letters, numbers, hyphens, underscores only
/// - Can't start or end with hyphen or underscore
pub fn validate_youtube_username(username: &str) -> Result<(), ()> {
    if username.len() < 3 || username.len() > 20 {
        return Err(());
    }
    
    if username.starts_with('-') || username.starts_with('_') ||
       username.ends_with('-') || username.ends_with('_') {
        return Err(());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(());
    }
    
    Ok(())
}

/// Validates TikTok username format
/// Rules:
/// - 1-24 characters
/// - Letters, numbers, underscores, periods
/// - Must start with a letter
/// - Can't have consecutive underscores
pub fn validate_tiktok_username(username: &str) -> Result<(), ()> {
    if username.is_empty() || username.len() > 24 {
        return Err(());
    }
    
    if !username.chars().next().map_or(false, |c| c.is_alphabetic()) {
        return Err(());
    }
    
    if username.contains("__") {
        return Err(());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
        return Err(());
    }
    
    Ok(())
}

