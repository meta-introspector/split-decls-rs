// Generated macro for Load (trait)
macro_rules! Depcrate_loadLoad {
() => {
// Module: crate::load
// Provides: {"Load"}
// Dependencies: {}
# [doc = " Types that implement this trait can give an estimate of how loaded they are."] # [doc = ""] # [doc = " See the module documentation for more details."] pub trait Load { # [doc = " A comparable load metric."] # [doc = ""] # [doc = " Lesser values indicate that the service is less loaded, and should be preferred for new"] # [doc = " requests over another service with a higher value."] type Metric : PartialOrd ; # [doc = " Estimate the service's current load."] fn load (& self) -> Self :: Metric ; }
};
}
