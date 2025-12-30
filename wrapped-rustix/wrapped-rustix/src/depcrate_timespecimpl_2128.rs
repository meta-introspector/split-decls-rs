// Generated macro for impl_2128 (impl)
macro_rules! Depcrate_timespecimpl_2128 {
() => {
// Module: crate::timespec
// Provides: {"impl_2128"}
// Dependencies: {}
impl Sub for Timespec { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . checked_sub (rhs) . expect ("overflow when subtracting timespecs") } }
};
}
