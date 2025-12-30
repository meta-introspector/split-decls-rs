// Generated macro for OwnedFd (struct)
macro_rules! Depcrate_os_solid_ioOwnedFd {
() => {
// Module: crate::os::solid::io
// Provides: {"OwnedFd"}
// Dependencies: {}
# [doc = " An owned SOLID Sockets file descriptor."] # [doc = ""] # [doc = " This closes the file descriptor on drop."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host file"] # [doc = " descriptor, so it can be used in FFI in places where a socket is passed as"] # [doc = " an argument, it is not captured or consumed, and it never has the value"] # [doc = " `SOLID_NET_INVALID_FD`."] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] pub struct OwnedFd { fd : ValidRawFd , }
};
}
