// Generated macro for linux_imports (module)
macro_rules! Depcrate_quic_io_gsolinux_imports {
() => {
// Module: crate::quic::io::gso
// Provides: {"linux_imports"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , not (feature = "fuzzing")))] mod linux_imports { pub (super) use nix :: sys :: socket :: sendmsg ; pub (super) use nix :: sys :: socket :: ControlMessage ; pub (super) use nix :: sys :: socket :: MsgFlags ; pub (super) use nix :: sys :: socket :: SockaddrStorage ; pub (super) use smallvec :: SmallVec ; pub (super) use std :: io :: ErrorKind ; pub (super) use std :: os :: fd :: AsRawFd ; pub (super) use tokio :: io :: Interest ; }
};
}
