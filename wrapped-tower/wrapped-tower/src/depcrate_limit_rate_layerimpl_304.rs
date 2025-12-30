// Generated macro for impl_304 (impl)
macro_rules! Depcrate_limit_rate_layerimpl_304 {
() => {
// Module: crate::limit::rate::layer
// Provides: {"impl_304"}
// Dependencies: {}
impl RateLimitLayer { # [doc = " Create new rate limit layer."] pub const fn new (num : u64 , per : Duration) -> Self { let rate = Rate :: new (num , per) ; RateLimitLayer { rate } } }
};
}
