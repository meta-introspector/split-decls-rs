// Generated macro for impl_51 (impl)
macro_rules! Depcrate_socketimpl_51 {
() => {
// Module: crate::socket
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < Socket > for UnixListener { fn from (socket : Socket) -> UnixListener { socket . inner . into () } }
};
}
