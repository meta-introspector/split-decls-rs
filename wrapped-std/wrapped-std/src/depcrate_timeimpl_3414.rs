// Generated macro for impl_3414 (impl)
macro_rules! Depcrate_timeimpl_3414 {
() => {
// Module: crate::time
// Provides: {"impl_3414"}
// Dependencies: {}
# [stable (feature = "time2" , since = "1.8.0")] impl Sub < Instant > for Instant { type Output = Duration ; # [doc = " Returns the amount of time elapsed from another instant to this one,"] # [doc = " or zero duration if that instant is later than this one."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Previous Rust versions panicked when `other` was later than `self`. Currently this"] # [doc = " method saturates. Future versions may reintroduce the panic in some circumstances."] # [doc = " See [Monotonicity]."] # [doc = ""] # [doc = " [Monotonicity]: Instant#monotonicity"] fn sub (self , other : Instant) -> Duration { self . duration_since (other) } }
};
}
