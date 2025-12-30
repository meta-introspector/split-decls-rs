// Generated macro for BorrowedFd (struct)
macro_rules! Depcrate_os_solid_ioBorrowedFd {
() => {
// Module: crate::os::solid::io
// Provides: {"BorrowedFd"}
// Dependencies: {}
# [doc = " A borrowed SOLID Sockets file descriptor."] # [doc = ""] # [doc = " This has a lifetime parameter to tie it to the lifetime of something that"] # [doc = " owns the socket."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host file"] # [doc = " descriptor, so it can be used in FFI in places where a socket is passed as"] # [doc = " an argument, it is not captured or consumed, and it never has the value"] # [doc = " `SOLID_NET_INVALID_FD`."] # [doc = ""] # [doc = " This type's `.to_owned()` implementation returns another `BorrowedFd`"] # [doc = " rather than an `OwnedFd`. It just makes a trivial copy of the raw"] # [doc = " socket, which is then borrowed under the same lifetime."] # [derive (Copy , Clone)] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] pub struct BorrowedFd < 'socket > { fd : ValidRawFd , _phantom : PhantomData < & 'socket OwnedFd > , }
};
}
