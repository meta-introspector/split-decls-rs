// Generated macro for impl_539 (impl)
macro_rules! Depcrate_retry_backoffimpl_539 {
() => {
// Module: crate::retry::backoff
// Provides: {"impl_539"}
// Dependencies: {}
impl Default for ExponentialBackoffMaker { fn default () -> Self { ExponentialBackoffMaker :: new (Duration :: from_millis (50) , Duration :: from_millis (u64 :: MAX) , 0.99 , HasherRng :: default () ,) . expect ("Unable to create ExponentialBackoff") } }
};
}
