// Generated macro for impl_44 (impl)
macro_rules! Depcrate_elaborateimpl_44 {
() => {
// Module: crate::elaborate
// Provides: {"impl_44"}
// Dependencies: {}
impl < I : Interner , O : Elaboratable < I > > Iterator for Elaborator < I , O > { type Item = O ; fn size_hint (& self) -> (usize , Option < usize >) { (self . stack . len () , None) } fn next (& mut self) -> Option < Self :: Item > { if let Some (obligation) = self . stack . pop () { self . elaborate (& obligation) ; Some (obligation) } else { None } } }
};
}
