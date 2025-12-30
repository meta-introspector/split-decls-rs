// Generated macro for impl_3412 (impl)
macro_rules! Depcrate_timeimpl_3412 {
() => {
// Module: crate::time
// Provides: {"impl_3412"}
// Dependencies: {}
# [stable (feature = "time2" , since = "1.8.0")] impl Sub < Duration > for Instant { type Output = Instant ; fn sub (self , other : Duration) -> Instant { self . checked_sub (other) . expect ("overflow when subtracting duration from instant") } }
};
}
