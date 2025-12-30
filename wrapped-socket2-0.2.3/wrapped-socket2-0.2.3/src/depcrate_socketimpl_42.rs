// Generated macro for impl_42 (impl)
macro_rules! Depcrate_socketimpl_42 {
() => {
// Module: crate::socket
// Provides: {"impl_42"}
// Dependencies: {}
impl From < net :: TcpListener > for Socket { fn from (socket : net :: TcpListener) -> Socket { Socket { inner : socket . into () } } }
};
}
