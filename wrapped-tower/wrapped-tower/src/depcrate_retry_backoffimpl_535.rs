// Generated macro for impl_535 (impl)
macro_rules! Depcrate_retry_backoffimpl_535 {
() => {
// Module: crate::retry::backoff
// Provides: {"impl_535"}
// Dependencies: {}
impl < R > ExponentialBackoffMaker < R > where R : Rng , { # [doc = " Create a new `ExponentialBackoff`."] # [doc = ""] # [doc = " # Error"] # [doc = ""] # [doc = " Returns a config validation error if:"] # [doc = " - `min` > `max`"] # [doc = " - `max` == 0"] # [doc = " - `jitter` < `0.0`"] # [doc = " - `jitter` > `100.0`"] # [doc = " - `jitter` is not finite"] pub fn new (min : time :: Duration , max : time :: Duration , jitter : f64 , rng : R ,) -> Result < Self , InvalidBackoff > { if min > max { return Err (InvalidBackoff ("maximum must not be less than minimum")) ; } if max == time :: Duration :: from_millis (0) { return Err (InvalidBackoff ("maximum must be non-zero")) ; } if jitter < 0.0 { return Err (InvalidBackoff ("jitter must not be negative")) ; } if jitter > 100.0 { return Err (InvalidBackoff ("jitter must not be greater than 100")) ; } if ! jitter . is_finite () { return Err (InvalidBackoff ("jitter must be finite")) ; } Ok (ExponentialBackoffMaker { min , max , jitter , rng , }) } }
};
}
