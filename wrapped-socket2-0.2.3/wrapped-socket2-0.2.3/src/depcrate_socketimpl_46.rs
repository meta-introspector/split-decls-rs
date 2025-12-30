// Generated macro for impl_46 (impl)
macro_rules! Depcrate_socketimpl_46 {
() => {
// Module: crate::socket
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < UnixDatagram > for Socket { fn from (socket : UnixDatagram) -> Socket { Socket { inner : socket . into () } } }
};
}
