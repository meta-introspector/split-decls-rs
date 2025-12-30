// Generated macro for try_cache_interest (function)
macro_rules! Depcrate_log_tracertry_cache_interest {
() => {
// Module: crate::log_tracer
// Provides: {"try_cache_interest"}
// Dependencies: {}
# [cfg (not (all (feature = "interest-cache" , feature = "std")))] fn try_cache_interest (_ : & log :: Metadata < '_ > , callback : impl FnOnce () -> bool) -> bool { callback () }
};
}
