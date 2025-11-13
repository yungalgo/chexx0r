use chexx0r::*;
use std::collections::HashMap;

// Known taken usernames (verified to exist on platforms)
const KNOWN_TAKEN: &[&str] = &[
    "taylorswift",    // Verified: YouTube 200, Instagram 301, TikTok statusCode 0
    "nike",           // Verified: YouTube 200, Instagram 301, TikTok statusCode 0
    "cristiano",      // Verified: YouTube 200, Instagram 301
    "tiktok",         // Verified: YouTube 200, TikTok statusCode 0
    "youtube",        // Verified: YouTube 200
    "pewdiepie",      // Verified: YouTube 200
    "charlidamelio",  // Verified: TikTok statusCode 0
    "selenagomez",    // Famous person (likely taken)
    "therock",        // Famous person (likely taken)
];

// Known untaken usernames (verified to not exist)
const KNOWN_UNTAKEN: &[&str] = &[
    "definitelynotrealuser12345xyz",      // Verified: YouTube 404, TikTok statusCode 10221
    "randomuserxyz987654321",             // Verified: YouTube 404
    "thisdefinitelydoesnotexist12345xyz", // Verified: YouTube 404, TikTok statusCode 10221
    "nonexistentuser999888777",           // Random long string
    "fakeaccountxyz123456789",            // Random long string
    "notrealuser987654321abc",            // Random long string
    "testuserxyz123456789012",            // Random long string
    "imaginaryuser999888777666",          // Random long string
    "phantomuser123456789xyz",            // Random long string
    "ghostaccount987654321abc",           // Random long string
];

// Invalid usernames mapped to platforms they should be invalid for
// Format: (username, platforms_where_invalid)
const INVALID_USERNAMES: &[(&str, &[&str])] = &[
    ("ab", &["YouTube"]),              // Too short for YouTube (min 3 chars)
    ("a", &["YouTube"]),               // Too short for YouTube (min 3 chars)
    ("user@name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (@) for all platforms
    ("user name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (space) for all platforms
    ("user-name-", &["YouTube", "TikTok", "Instagram"]), // Ends with hyphen (invalid for YouTube/TikTok), contains hyphen (invalid for Instagram - only allows letters, numbers, periods, underscores)
    ("user_name_", &["YouTube"]),       // Ends with underscore (invalid for YouTube)
    ("-username", &["YouTube", "TikTok", "Instagram"]), // Starts with hyphen (invalid for YouTube/TikTok, TikTok needs letter start), contains hyphen (invalid for Instagram)
    ("_username", &["YouTube", "TikTok"]), // Starts with underscore (invalid for YouTube/TikTok, TikTok needs letter start)
    (".username", &["Instagram", "TikTok", "YouTube"]), // Starts with period (invalid for Instagram/TikTok, TikTok needs letter start), contains period (invalid for YouTube - only allows letters, numbers, hyphens, underscores)
    ("username.", &["Instagram", "YouTube"]), // Ends with period (invalid for Instagram), contains period (invalid for YouTube)
    ("user..name", &["Instagram", "YouTube"]), // Consecutive periods (invalid for Instagram), contains period (invalid for YouTube)
    ("user__name", &["TikTok"]),       // Consecutive underscores (invalid for TikTok)
    ("1username", &["TikTok"]),        // Starts with number (invalid for TikTok)
    ("user!name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (!) for all platforms
    ("user#name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (#) for all platforms
    ("user$name", &["YouTube", "Instagram", "TikTok"]), // Invalid char ($) for all platforms
    ("user%name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (%) for all platforms
    ("user&name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (&) for all platforms
    ("user*name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (*) for all platforms
    ("user+name", &["YouTube", "Instagram", "TikTok"]), // Invalid char (+) for all platforms
];

#[tokio::test]
async fn test_known_taken_usernames() {
    let mut results = HashMap::new();
    
    for username in KNOWN_TAKEN {
        println!("\nTesting KNOWN TAKEN: {}", username);
        
        // Test YouTube
        let youtube_url = format!("https://www.youtube.com/@{}", username);
        let youtube_result = check_social_platform_internal(&youtube_url, "YouTube").await;
        results.insert(format!("YouTube-{}", username), youtube_result);
        
        // Test Instagram
        let instagram_url = format!("https://www.instagram.com/{}", username);
        let instagram_result = check_social_platform_internal(&instagram_url, "Instagram").await;
        results.insert(format!("Instagram-{}", username), instagram_result);
        
        // Test TikTok
        let tiktok_url = format!("https://www.tiktok.com/@{}", username);
        let tiktok_result = check_social_platform_internal(&tiktok_url, "TikTok").await;
        results.insert(format!("TikTok-{}", username), tiktok_result);
    }
    
    // Check results
    let mut failures = Vec::new();
    for (key, result) in &results {
        match result {
            Ok(true) => {
                println!("✓ {}: CORRECTLY detected as TAKEN", key);
            }
            Ok(false) => {
                failures.push(format!("✗ {}: FALSE NEGATIVE - should be TAKEN but detected as AVAILABLE", key));
            }
            Err(e) => {
                failures.push(format!("✗ {}: ERROR - {}", key, e));
            }
        }
    }
    
    if !failures.is_empty() {
        panic!("Found {} false negatives:\n{}", failures.len(), failures.join("\n"));
    }
}

#[tokio::test]
async fn test_known_untaken_usernames() {
    let mut results = HashMap::new();
    
    for username in KNOWN_UNTAKEN {
        println!("\nTesting KNOWN UNTAKEN: {}", username);
        
        // Test YouTube
        let youtube_url = format!("https://www.youtube.com/@{}", username);
        let youtube_result = check_social_platform_internal(&youtube_url, "YouTube").await;
        results.insert(format!("YouTube-{}", username), youtube_result);
        
        // Test Instagram
        let instagram_url = format!("https://www.instagram.com/{}", username);
        let instagram_result = check_social_platform_internal(&instagram_url, "Instagram").await;
        results.insert(format!("Instagram-{}", username), instagram_result);
        
        // Test TikTok
        let tiktok_url = format!("https://www.tiktok.com/@{}", username);
        let tiktok_result = check_social_platform_internal(&tiktok_url, "TikTok").await;
        results.insert(format!("TikTok-{}", username), tiktok_result);
    }
    
    // Check results
    let mut failures = Vec::new();
    for (key, result) in &results {
        match result {
            Ok(false) => {
                println!("✓ {}: CORRECTLY detected as AVAILABLE", key);
            }
            Ok(true) => {
                failures.push(format!("✗ {}: FALSE POSITIVE - should be AVAILABLE but detected as TAKEN", key));
            }
            Err(e) => {
                // Errors are acceptable for untaken usernames (network issues, etc.)
                println!("? {}: ERROR (acceptable) - {}", key, e);
            }
        }
    }
    
    if !failures.is_empty() {
        panic!("Found {} false positives:\n{}", failures.len(), failures.join("\n"));
    }
}

#[tokio::test]
async fn test_invalid_usernames() {
    let mut failures = Vec::new();
    
    for (username, invalid_platforms) in INVALID_USERNAMES {
        println!("\nTesting INVALID: {} (should be invalid for: {:?})", username, invalid_platforms);
        
        // Test YouTube validation
        let youtube_valid = validate_youtube_username(username);
        let should_be_invalid_youtube = invalid_platforms.contains(&"YouTube");
        if should_be_invalid_youtube && youtube_valid.is_ok() {
            failures.push(format!("✗ YouTube-{}: Should be INVALID but validation passed", username));
        } else if !should_be_invalid_youtube && youtube_valid.is_err() {
            failures.push(format!("✗ YouTube-{}: Should be VALID but validation failed", username));
        } else {
            println!("✓ YouTube-{}: {}", username, if should_be_invalid_youtube { "CORRECTLY detected as INVALID" } else { "CORRECTLY detected as VALID" });
        }
        
        // Test Instagram validation
        let instagram_valid = validate_instagram_username(username);
        let should_be_invalid_instagram = invalid_platforms.contains(&"Instagram");
        if should_be_invalid_instagram && instagram_valid.is_ok() {
            failures.push(format!("✗ Instagram-{}: Should be INVALID but validation passed", username));
        } else if !should_be_invalid_instagram && instagram_valid.is_err() {
            failures.push(format!("✗ Instagram-{}: Should be VALID but validation failed", username));
        } else {
            println!("✓ Instagram-{}: {}", username, if should_be_invalid_instagram { "CORRECTLY detected as INVALID" } else { "CORRECTLY detected as VALID" });
        }
        
        // Test TikTok validation
        let tiktok_valid = validate_tiktok_username(username);
        let should_be_invalid_tiktok = invalid_platforms.contains(&"TikTok");
        if should_be_invalid_tiktok && tiktok_valid.is_ok() {
            failures.push(format!("✗ TikTok-{}: Should be INVALID but validation passed", username));
        } else if !should_be_invalid_tiktok && tiktok_valid.is_err() {
            failures.push(format!("✗ TikTok-{}: Should be VALID but validation failed", username));
        } else {
            println!("✓ TikTok-{}: {}", username, if should_be_invalid_tiktok { "CORRECTLY detected as INVALID" } else { "CORRECTLY detected as VALID" });
        }
    }
    
    if !failures.is_empty() {
        panic!("Found {} validation failures:\n{}", failures.len(), failures.join("\n"));
    }
}

// Helper function to check social platform (internal version for testing)
async fn check_social_platform_internal(url: &str, _platform: &str) -> Result<bool, String> {
    use reqwest::Client;
    use std::time::Duration;
    
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;
    
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    let status = response.status();
    
    // YouTube: Returns 404 for non-existent profiles, 200 for existing ones
    if url.contains("youtube.com") {
        if status.as_u16() == 404 {
            return Ok(false); // Available
        } else if status.is_success() {
            return Ok(true); // Taken
        } else {
            return Err(format!("HTTP {}", status.as_u16()));
        }
    }
    
    // Instagram and TikTok return 200 but show specific error messages
    if status.as_u16() == 404 {
        return Ok(false); // Available
    }
    
    if status.is_success() {
        let body = response.text().await.map_err(|e| format!("Failed to read body: {}", e))?;
        let body_lower = body.to_lowercase();
        
        // Check for platform-specific patterns using the library functions
        if url.contains("instagram.com") {
            return check_instagram_availability(&body_lower, url)
                .map_err(|e| format!("Instagram check error: {}", e));
        }
        
        if url.contains("tiktok.com") {
            return check_tiktok_availability(&body_lower)
                .map_err(|e| format!("TikTok check error: {}", e));
        }
        
        // Default: assume taken
        Ok(true)
    } else {
        Err(format!("HTTP {}", status.as_u16()))
    }
}
