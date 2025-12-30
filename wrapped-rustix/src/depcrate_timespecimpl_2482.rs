// Generated macro for impl_2482 (impl)
macro_rules! Depcrate_timespecimpl_2482 {
() => {
// Module: crate::timespec
// Provides: {"impl_2482"}
// Dependencies: {}
impl Sub for Timespec { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . checked_sub (rhs) . expect ("overflow when subtracting timespecs") } }
};
}
