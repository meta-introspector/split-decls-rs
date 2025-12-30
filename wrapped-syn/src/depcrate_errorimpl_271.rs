// Generated macro for impl_271 (impl)
macro_rules! Depcrate_errorimpl_271 {
() => {
// Module: crate::error
// Provides: {"impl_271"}
// Dependencies: {}
impl Iterator for IntoIter { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?] , }) } }
};
}
