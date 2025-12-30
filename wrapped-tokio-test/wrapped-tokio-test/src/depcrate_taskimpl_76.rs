// Generated macro for impl_76 (impl)
macro_rules! Depcrate_taskimpl_76 {
() => {
// Module: crate::task
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : Stream > Stream for Spawn < T > { type Item = T :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . future . as_mut () . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . future . size_hint () } }
};
}
