// Generated macro for impl_42 (impl)
macro_rules! Depcrate_time_system_timeimpl_42 {
() => {
// Module: crate::time::system_time
// Provides: {"impl_42"}
// Dependencies: {}
impl Sub < Duration > for SystemTime { type Output = Self ; fn sub (self , rhs : Duration) -> Self { self . checked_sub (rhs) . expect ("overflow when subtracting duration from instant") } }
};
}
