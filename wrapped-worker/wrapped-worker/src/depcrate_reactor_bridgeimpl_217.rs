// Generated macro for impl_217 (impl)
macro_rules! Depcrate_reactor_bridgeimpl_217 {
() => {
// Module: crate::reactor::bridge
// Provides: {"impl_217"}
// Dependencies: {}
impl < R > FusedStream for ReactorBridge < R > where R : Reactor + 'static , { fn is_terminated (& self) -> bool { self . rx . is_terminated () } }
};
}
