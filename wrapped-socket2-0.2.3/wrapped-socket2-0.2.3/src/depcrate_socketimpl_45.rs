// Generated macro for impl_45 (impl)
macro_rules! Depcrate_socketimpl_45 {
() => {
// Module: crate::socket
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (all (unix , feature = "unix"))] impl From < UnixListener > for Socket { fn from (socket : UnixListener) -> Socket { Socket { inner : socket . into () } } }
};
}
