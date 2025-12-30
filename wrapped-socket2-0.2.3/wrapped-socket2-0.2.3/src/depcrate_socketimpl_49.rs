// Generated macro for impl_49 (impl)
macro_rules! Depcrate_socketimpl_49 {
() => {
// Module: crate::socket
// Provides: {"impl_49"}
// Dependencies: {}
impl From < Socket > for net :: UdpSocket { fn from (socket : Socket) -> net :: UdpSocket { socket . inner . into () } }
};
}
