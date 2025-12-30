// Generated macro for nanos (function)
macro_rules! Depcrate_hedge_rotating_histogramnanos {
() => {
// Module: crate::hedge::rotating_histogram
// Provides: {"nanos"}
// Dependencies: {}
fn nanos (duration : Duration) -> u64 { duration . as_secs () . saturating_mul (NANOS_PER_SEC) . saturating_add (u64 :: from (duration . subsec_nanos ())) }
};
}
