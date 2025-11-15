/// Domain availability checking functionality
/// 
/// EXPRESS CONCERNS:
/// - Checking domain availability across multiple TLDs
/// - Integrating with domain_check_lib
/// - Returning structured domain results
/// 
/// DOES NOT:
/// - Render UI or format output
/// - Handle CLI arguments
/// - Manage TLD presets (delegates to config module)

use anyhow::Result;
use domain_check_lib::DomainChecker;
use crate::config::get_preset_tlds;

/// Domain check result
pub struct DomainResult {
    pub domain: String,
    pub available: Option<bool>, // Some(true) = available, Some(false) = taken, None = unknown
}

/// Check domain availability for a username across multiple TLDs
/// Returns a vector of domain results - NO UI rendering
pub async fn check_domains(username: &str, preset: &str, custom_tlds: Option<&str>) -> Result<Vec<DomainResult>> {
    let tlds = if let Some(tlds_str) = custom_tlds {
        tlds_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        get_preset_tlds(preset)
    };

    let checker = DomainChecker::new();
    let mut results = Vec::new();

    for tld in &tlds {
        let domain = format!("{}.{}", username, tld);
        
        match checker.check_domain(&domain).await {
            Ok(result) => {
                results.push(DomainResult {
                    domain,
                    available: result.available,
                });
            }
            Err(_) => {
                results.push(DomainResult {
                    domain,
                    available: None,
                });
            }
        }
    }

    Ok(results)
}
