// Generated macro for impl_43 (impl)
macro_rules! Depcrate_socketimpl_43 {
() => {
// Module: crate::socket
// Provides: {"impl_43"}
// Dependencies: {}
impl From < net :: UdpSocket > for Socket { fn from (socket : net :: UdpSocket) -> Socket { Socket { inner : socket . into () } } }
};
}
