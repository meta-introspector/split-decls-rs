// Generated macro for SockAddrStorage (struct)
macro_rules! Depcrate_sockaddrSockAddrStorage {
() => {
// Module: crate::sockaddr
// Provides: {"SockAddrStorage"}
// Dependencies: {}
# [doc = " Rust version of the [`sockaddr_storage`] type."] # [doc = ""] # [doc = " This type is intended to be used with with direct calls to the `getsockname` syscall. See the"] # [doc = " documentation of [`SockAddr::new`] for examples."] # [doc = ""] # [doc = " This crate defines its own `sockaddr_storage` type to avoid semver concerns with upgrading"] # [doc = " `windows-sys`."] # [repr (transparent)] pub struct SockAddrStorage { storage : sockaddr_storage , }
};
}
