// Generated macro for Builder (struct)
macro_rules! Depcrate_log_tracerBuilder {
() => {
// Module: crate::log_tracer
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " Configures a new `LogTracer`."] # [derive (Debug)] pub struct Builder { ignore_crates : Vec < String > , filter : log :: LevelFilter , # [cfg (all (feature = "interest-cache" , feature = "std"))] interest_cache_config : Option < crate :: InterestCacheConfig > , }
};
}
