// Generated macro for impl_3419 (impl)
macro_rules! Depcrate_timeimpl_3419 {
() => {
// Module: crate::time
// Provides: {"impl_3419"}
// Dependencies: {}
# [stable (feature = "time2" , since = "1.8.0")] impl Sub < Duration > for SystemTime { type Output = SystemTime ; fn sub (self , dur : Duration) -> SystemTime { self . checked_sub (dur) . expect ("overflow when subtracting duration from instant") } }
};
}
