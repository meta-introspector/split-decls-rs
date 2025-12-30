// Generated macro for nanos (function)
macro_rules! Depcrate_load_peak_ewmananos {
() => {
// Module: crate::load::peak_ewma
// Provides: {"nanos"}
// Dependencies: {}
fn nanos (d : Duration) -> f64 { const NANOS_PER_SEC : u64 = 1_000_000_000 ; let n = f64 :: from (d . subsec_nanos ()) ; let s = d . as_secs () . saturating_mul (NANOS_PER_SEC) as f64 ; n + s }
};
}
