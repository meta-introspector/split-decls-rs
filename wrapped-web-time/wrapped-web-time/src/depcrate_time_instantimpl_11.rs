// Generated macro for impl_11 (impl)
macro_rules! Depcrate_time_instantimpl_11 {
() => {
// Module: crate::time::instant
// Provides: {"impl_11"}
// Dependencies: {}
impl Sub < Duration > for Instant { type Output = Self ; fn sub (self , rhs : Duration) -> Self { self . checked_sub (rhs) . expect ("overflow when subtracting duration from instant") } }
};
}
