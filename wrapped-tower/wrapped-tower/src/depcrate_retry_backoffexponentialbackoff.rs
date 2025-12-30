// Generated macro for ExponentialBackoff (struct)
macro_rules! Depcrate_retry_backoffExponentialBackoff {
() => {
// Module: crate::retry::backoff
// Provides: {"ExponentialBackoff"}
// Dependencies: {}
# [doc = " A jittered [exponential backoff] strategy."] # [doc = ""] # [doc = " The backoff duration will increase exponentially for every subsequent"] # [doc = " backoff, up to a maximum duration. A small amount of [random jitter] is"] # [doc = " added to each backoff duration, in order to avoid retry spikes."] # [doc = ""] # [doc = " [exponential backoff]: https://en.wikipedia.org/wiki/Exponential_backoff"] # [doc = " [random jitter]: https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/"] # [derive (Debug , Clone)] pub struct ExponentialBackoff < R = HasherRng > { min : time :: Duration , max : time :: Duration , jitter : f64 , rng : R , iterations : u32 , }
};
}
