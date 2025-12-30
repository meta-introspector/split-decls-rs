// Generated macro for impl_216 (impl)
macro_rules! Depcrate_reactor_bridgeimpl_216 {
() => {
// Module: crate::reactor::bridge
// Provides: {"impl_216"}
// Dependencies: {}
impl < R > Stream for ReactorBridge < R > where R : Reactor + 'static , { type Item = < R :: Scope as ReactorScoped > :: Output ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut self . rx) . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . rx . size_hint () } }
};
}
