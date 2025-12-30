// Generated macro for impl_272 (impl)
macro_rules! Depcrate_errorimpl_272 {
() => {
// Module: crate::error
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?. clone ()] , }) } }
};
}
