// Generated macro for macro_408 (macro)
macro_rules! Depcrate_util_tracemacro_408 {
() => {
// Module: crate::util::trace
// Provides: {"macro_408"}
// Dependencies: {}
cfg_time ! { # [track_caller] pub (crate) fn caller_location () -> Option <&'static std :: panic :: Location <'static >> { # [cfg (all (tokio_unstable , feature = "tracing"))] return Some (std :: panic :: Location :: caller ()) ; # [cfg (not (all (tokio_unstable , feature = "tracing")))] None } }
};
}
