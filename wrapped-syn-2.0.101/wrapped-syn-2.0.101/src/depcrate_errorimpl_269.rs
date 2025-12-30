// Generated macro for impl_269 (impl)
macro_rules! Depcrate_errorimpl_269 {
() => {
// Module: crate::error
// Provides: {"impl_269"}
// Dependencies: {}
impl Iterator for IntoIter { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { Some (Error { messages : vec ! [self . messages . next () ?] , }) } }
};
}
