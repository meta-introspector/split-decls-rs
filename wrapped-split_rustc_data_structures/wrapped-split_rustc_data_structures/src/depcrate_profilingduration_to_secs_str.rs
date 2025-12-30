// Generated macro for duration_to_secs_str (function)
macro_rules! Depcrate_profilingduration_to_secs_str {
() => {
// Module: crate::profiling
// Provides: {"duration_to_secs_str"}
// Dependencies: {}
pub fn duration_to_secs_str (dur : std :: time :: Duration) -> String { format ! ("{:.3}" , dur . as_secs_f64 ()) }
};
}
