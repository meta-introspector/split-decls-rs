// Generated macro for get_concurrency (function)
macro_rules! Depcrate_helpers_concurrencyget_concurrency {
() => {
// Module: crate::helpers::concurrency
// Provides: {"get_concurrency"}
// Dependencies: {}
pub (crate) fn get_concurrency () -> usize { if let Ok (value) = env :: var ("RUST_TEST_THREADS") { match value . parse :: < NonZero < usize > > () . ok () { Some (n) => n . get () , _ => panic ! ("RUST_TEST_THREADS is `{value}`, should be a positive integer.") , } } else { thread :: available_parallelism () . map (| n | n . get ()) . unwrap_or (1) } }
};
}
