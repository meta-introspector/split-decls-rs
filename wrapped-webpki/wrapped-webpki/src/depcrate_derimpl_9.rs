// Generated macro for impl_9 (impl)
macro_rules! Depcrate_derimpl_9 {
() => {
// Module: crate::der
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , T : FromDer < 'a > > Iterator for DerIterator < 'a , T > { type Item = Result < T , Error > ; fn next (& mut self) -> Option < Self :: Item > { (! self . reader . at_end ()) . then (| | T :: from_der (& mut self . reader)) } }
};
}
