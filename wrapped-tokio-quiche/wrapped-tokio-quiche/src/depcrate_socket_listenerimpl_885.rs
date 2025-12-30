// Generated macro for impl_885 (impl)
macro_rules! Depcrate_socket_listenerimpl_885 {
() => {
// Module: crate::socket::listener
// Provides: {"impl_885"}
// Dependencies: {}
impl QuicListener { # [doc = " Tries to enable all sockopts supported by the crate for this socket."] # [doc = " See `SocketCapabilities::apply_all_and_get_compatibility` for details."] # [cfg (target_os = "linux")] pub fn apply_max_capabilities (& mut self) { let capabilities = SocketCapabilities :: apply_all_and_get_compatibility (& self . socket) ; self . capabilities = capabilities ; } }
};
}
