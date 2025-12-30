// Generated macro for impl_47 (impl)
macro_rules! Depcrate_socketimpl_47 {
() => {
// Module: crate::socket
// Provides: {"impl_47"}
// Dependencies: {}
impl From < Socket > for net :: TcpStream { fn from (socket : Socket) -> net :: TcpStream { socket . inner . into () } }
};
}
