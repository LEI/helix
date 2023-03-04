//! This modules provides a simple enum type to represent the trust status of a document.
//! The security feature ensures no unauthorized code execution can take place.
//! without an explicit activation by the user.
//!
//! This behavior can be disabled by setting `enable = false` under `[security]`,
//! otherwise all documents not listed in `trusted` are restricted by default.
//! Scratch buffers are trusted unless `trust_scratch_buffer` is set to `false`.

// FIXME:
// - preserve doc state e.g. on :split
// -> should be resolve with correct project directory handling
// TODO:
// - handle workspace directory instead of a single document
// - enable some LSP features, maybe restict debugging?
// - persist user choices over restarts (update config.toml?)

/// Trust status, also known as workspace mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustStatus {
    /// Enable all features, equivalent to `true`.
    Trusted,
    /// Safe code browsing, equivalent to `false`.
    Restricted,
}

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

const TRUSTED: bool = true;
const RESTRICTED: bool = false;

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
