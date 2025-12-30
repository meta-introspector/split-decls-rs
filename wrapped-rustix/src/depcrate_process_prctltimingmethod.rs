// Generated macro for TimingMethod (enum)
macro_rules! Depcrate_process_prctlTimingMethod {
() => {
// Module: crate::process::prctl
// Provides: {"TimingMethod"}
// Dependencies: {}
# [doc = " `PR_TIMING_*` values for use with [`timing_method`] and"] # [doc = " [`set_timing_method`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (i32)] pub enum TimingMethod { # [doc = " Normal, traditional, statistical process timing."] Statistical = PR_TIMING_STATISTICAL , # [doc = " Accurate timestamp based process timing."] TimeStamp = PR_TIMING_TIMESTAMP , }
};
}
