// Generated macro for impl_2480 (impl)
macro_rules! Depcrate_timespecimpl_2480 {
() => {
// Module: crate::timespec
// Provides: {"impl_2480"}
// Dependencies: {}
impl Add for Timespec { type Output = Self ; fn add (self , rhs : Self) -> Self { self . checked_add (rhs) . expect ("overflow when adding timespecs") } }
};
}
