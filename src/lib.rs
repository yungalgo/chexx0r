/// Library crate for chexx0r - exposes modules for testing
/// 
/// EXPRESS CONCERNS:
/// - Re-exporting public APIs for external use
/// - Making internal modules available for integration tests
/// - Providing a clean public interface
/// 
/// DOES NOT:
/// - Contain any business logic
/// - Render UI
/// - Perform checks directly

pub mod config;
pub mod domain;
pub mod social;
pub mod utils;
pub mod ui;

// Re-export commonly used functions and types for easier testing
pub use utils::{
    validate_instagram_username,
    validate_youtube_username,
    validate_tiktok_username,
};

pub use social::{
    check_instagram_availability,
    check_tiktok_availability,
    SocialResult,
    SocialStatus,
};

pub use domain::{
    DomainResult,
};

