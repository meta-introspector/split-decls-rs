// Generated macro for impl_16 (impl)
macro_rules! Depcrate_log_tracerimpl_16 {
() => {
// Module: crate::log_tracer
// Provides: {"impl_16"}
// Dependencies: {}
impl Default for Builder { fn default () -> Self { Self { ignore_crates : Vec :: new () , filter : log :: LevelFilter :: max () , # [cfg (all (feature = "interest-cache" , feature = "std"))] interest_cache_config : None , } } }
};
}
