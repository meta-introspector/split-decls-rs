// Generated macro for duration_nanos (function)
macro_rules! Depcrateduration_nanos {
() => {
// Module: crate
// Provides: {"duration_nanos"}
// Dependencies: {}
fn duration_nanos (d : Duration) -> f64 { (d . as_secs () as f64) + f64 :: from (d . subsec_nanos ()) / 1e9 }
};
}
