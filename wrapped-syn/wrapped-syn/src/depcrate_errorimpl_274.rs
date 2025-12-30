// Generated macro for impl_274 (impl)
macro_rules! Depcrate_errorimpl_274 {
() => {
// Module: crate::error
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?. clone ()] , }) } }
};
}
