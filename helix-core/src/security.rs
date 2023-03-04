/// Workspace mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustStatus {
    /// Enable all features
    Trusted,
    /// Safe code browsing
    Restricted,
}

const TRUSTED: bool = true;
const RESTRICTED: bool = false;

impl Default for TrustStatus {
    fn default() -> Self {
        Self::Restricted
    }
}

impl std::fmt::Display for TrustStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<bool> for TrustStatus {
    fn from(is_trusted: bool) -> Self {
        match is_trusted {
            TRUSTED => Self::Trusted,
            RESTRICTED => Self::Restricted,
        }
    }
}

impl From<TrustStatus> for bool {
    fn from(status: TrustStatus) -> bool {
        match status {
            TrustStatus::Trusted => TRUSTED,
            TrustStatus::Restricted => RESTRICTED,
        }
    }
}
