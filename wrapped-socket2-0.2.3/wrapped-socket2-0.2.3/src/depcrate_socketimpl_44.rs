// Generated macro for impl_44 (impl)
macro_rules! Depcrate_socketimpl_44 {
() => {
// Module: crate::socket
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < UnixStream > for Socket { fn from (socket : UnixStream) -> Socket { Socket { inner : socket . into () } } }
};
}
