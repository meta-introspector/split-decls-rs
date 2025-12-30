// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < I : Iterator > Iterator for Splice < '_ , I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
};
}
