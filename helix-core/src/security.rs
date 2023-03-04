/// Workspace mode
#[derive(Debug)]
pub enum TrustStatus {
    // Enable all features
    Trusted,
    // Safe code browsing
    Restricted,
}

impl Default for TrustStatus {
    fn default() -> Self {
        TrustStatus::Restricted
    }
}

impl From<bool> for TrustStatus {
    fn from(is_trusted: bool) -> Self {
        match is_trusted {
            true => TrustStatus::Trusted,
            false => TrustStatus::Restricted,
        }
    }
}
