// Generated macro for impl_959 (impl)
macro_rules! Depcrate_timeout_bodyimpl_959 {
() => {
// Module: crate::timeout::body
// Provides: {"impl_959"}
// Dependencies: {}
impl < B > TimeoutBody < B > { # [doc = " Creates a new [`TimeoutBody`]."] pub fn new (timeout : Duration , body : B) -> Self { TimeoutBody { timeout , sleep : None , body , } } }
};
}
