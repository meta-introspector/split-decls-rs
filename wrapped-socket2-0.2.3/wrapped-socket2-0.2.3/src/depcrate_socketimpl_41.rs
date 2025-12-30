// Generated macro for impl_41 (impl)
macro_rules! Depcrate_socketimpl_41 {
() => {
// Module: crate::socket
// Provides: {"impl_41"}
// Dependencies: {}
impl From < net :: TcpStream > for Socket { fn from (socket : net :: TcpStream) -> Socket { Socket { inner : socket . into () } } }
};
}
