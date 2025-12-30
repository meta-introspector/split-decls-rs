// Generated macro for impl_52 (impl)
macro_rules! Depcrate_socketimpl_52 {
() => {
// Module: crate::socket
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < Socket > for UnixDatagram { fn from (socket : Socket) -> UnixDatagram { socket . inner . into () } }
};
}
