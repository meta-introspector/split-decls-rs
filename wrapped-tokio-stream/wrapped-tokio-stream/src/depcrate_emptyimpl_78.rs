// Generated macro for impl_78 (impl)
macro_rules! Depcrate_emptyimpl_78 {
() => {
// Module: crate::empty
// Provides: {"impl_78"}
// Dependencies: {}
impl < T > Stream for Empty < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Ready (None) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
