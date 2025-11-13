/// Configuration constants and platform definitions

#[derive(Debug)]
pub struct SocialPlatform {
    pub name: &'static str,
    pub url_template: &'static str,
}

pub const SOCIAL_PLATFORMS: &[SocialPlatform] = &[
    SocialPlatform {
        name: "youtube",
        url_template: "https://www.youtube.com/@{}",
    },
    SocialPlatform {
        name: "instagram",
        url_template: "https://www.instagram.com/{}",
    },
    SocialPlatform {
        name: "tiktok",
        url_template: "https://www.tiktok.com/@{}",
    },
];

/// Get TLD list based on preset name
pub fn get_preset_tlds(preset: &str) -> Vec<String> {
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

