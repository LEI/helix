/// Workspace mode
#[derive(Debug, Clone, Copy)]
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
            true => Self::Trusted,
            false => Self::Restricted,
        }
    }
}

impl From<TrustStatus> for bool {
    fn from(status: TrustStatus) -> bool {
        match status {
            TrustStatus::Trusted => true,
            TrustStatus::Restricted => false,
        }
    }
}
