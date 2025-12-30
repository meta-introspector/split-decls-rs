// Generated macro for impl_2126 (impl)
macro_rules! Depcrate_timespecimpl_2126 {
() => {
// Module: crate::timespec
// Provides: {"impl_2126"}
// Dependencies: {}
impl Add for Timespec { type Output = Self ; fn add (self , rhs : Self) -> Self { self . checked_add (rhs) . expect ("overflow when adding timespecs") } }
};
}
