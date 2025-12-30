// Generated macro for impl_48 (impl)
macro_rules! Depcrate_socketimpl_48 {
() => {
// Module: crate::socket
// Provides: {"impl_48"}
// Dependencies: {}
impl From < Socket > for net :: TcpListener { fn from (socket : Socket) -> net :: TcpListener { socket . inner . into () } }
};
}
