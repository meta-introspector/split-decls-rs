// Generated macro for is_below_max_level (function)
macro_rules! Depcrate_filter_filter_fnis_below_max_level {
() => {
// Module: crate::filter::filter_fn
// Provides: {"is_below_max_level"}
// Dependencies: {}
fn is_below_max_level (hint : & Option < LevelFilter > , metadata : & Metadata < '_ >) -> bool { hint . as_ref () . map (| hint | metadata . level () <= hint) . unwrap_or (true) }
};
}
