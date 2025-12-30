// Generated macro for ExponentialBackoffMaker (struct)
macro_rules! Depcrate_retry_backoffExponentialBackoffMaker {
() => {
// Module: crate::retry::backoff
// Provides: {"ExponentialBackoffMaker"}
// Dependencies: {}
# [doc = " A maker type for [`ExponentialBackoff`]."] # [derive (Debug , Clone)] pub struct ExponentialBackoffMaker < R = HasherRng > { # [doc = " The minimum amount of time to wait before resuming an operation."] min : time :: Duration , # [doc = " The maximum amount of time to wait before resuming an operation."] max : time :: Duration , # [doc = " The ratio of the base timeout that may be randomly added to a backoff."] # [doc = ""] # [doc = " Must be greater than or equal to 0.0."] jitter : f64 , rng : R , }
};
}
