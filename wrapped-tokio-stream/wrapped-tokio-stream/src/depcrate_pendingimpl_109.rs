// Generated macro for impl_109 (impl)
macro_rules! Depcrate_pendingimpl_109 {
() => {
// Module: crate::pending
// Provides: {"impl_109"}
// Dependencies: {}
impl < T > Stream for Pending < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Pending } fn size_hint (& self) -> (usize , Option < usize >) { (0 , None) } }
};
}
