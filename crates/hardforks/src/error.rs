
#[derive(Debug)]

/// Error type for hardfork related errors.
pub struct HardforkError(pub alloc::string::String);

impl core::fmt::Display for HardforkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::error::Error for HardforkError {}
