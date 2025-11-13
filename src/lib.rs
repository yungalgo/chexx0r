/// Library crate for chexx0r - exposes modules for testing

pub mod config;
pub mod domain;
pub mod social;
pub mod utils;
pub mod ui;

// Re-export commonly used functions for easier testing
pub use utils::{
    validate_instagram_username,
    validate_youtube_username,
    validate_tiktok_username,
};

pub use social::{
    check_instagram_availability,
    check_tiktok_availability,
};

