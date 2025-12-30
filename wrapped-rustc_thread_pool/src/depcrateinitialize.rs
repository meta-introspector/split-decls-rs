// Generated macro for initialize (function)
macro_rules! Depcrateinitialize {
() => {
// Module: crate
// Provides: {"initialize"}
// Dependencies: {}
# [doc = " Deprecated in favor of `ThreadPoolBuilder::build_global`."] # [deprecated (note = "use `ThreadPoolBuilder::build_global`")] # [allow (deprecated)] pub fn initialize (config : Configuration) -> Result < () , Box < dyn Error > > { config . into_builder () . build_global () . map_err (Box :: from) }
};
}
