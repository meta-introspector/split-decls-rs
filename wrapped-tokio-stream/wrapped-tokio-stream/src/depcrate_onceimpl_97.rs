// Generated macro for impl_97 (impl)
macro_rules! Depcrate_onceimpl_97 {
() => {
// Module: crate::once
// Provides: {"impl_97"}
// Dependencies: {}
impl < T > Stream for Once < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { Pin :: new (& mut self . iter) . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
