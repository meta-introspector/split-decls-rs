// Generated macro for StartedKeyExchange (enum)
macro_rules! Depcrate_cryptoStartedKeyExchange {
() => {
// Module: crate::crypto
// Provides: {"StartedKeyExchange"}
// Dependencies: {}
# [doc = " Return value from [`SupportedKxGroup::start()`]."] # [non_exhaustive] pub enum StartedKeyExchange { # [doc = " A single [`ActiveKeyExchange`]."] Single (Box < dyn ActiveKeyExchange >) , # [doc = " A [`HybridKeyExchange`] that can potentially be split."] Hybrid (Box < dyn HybridKeyExchange >) , }
};
}
