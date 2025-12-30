// Generated macro for impl_50 (impl)
macro_rules! Depcrate_socketimpl_50 {
() => {
// Module: crate::socket
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < Socket > for UnixStream { fn from (socket : Socket) -> UnixStream { socket . inner . into () } }
};
}
