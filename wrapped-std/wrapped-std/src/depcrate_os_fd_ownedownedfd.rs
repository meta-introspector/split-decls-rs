// Generated macro for OwnedFd (struct)
macro_rules! Depcrate_os_fd_ownedOwnedFd {
() => {
// Module: crate::os::fd::owned
// Provides: {"OwnedFd"}
// Dependencies: {}
# [doc = " An owned file descriptor."] # [doc = ""] # [doc = " This closes the file descriptor on drop. It is guaranteed that nobody else will close the file"] # [doc = " descriptor."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host file"] # [doc = " descriptor, so it can be used in FFI in places where a file descriptor is"] # [doc = " passed as a consumed argument or returned as an owned value, and it never"] # [doc = " has the value `-1`."] # [doc = ""] # [doc = " You can use [`AsFd::as_fd`] to obtain a [`BorrowedFd`]."] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] # [stable (feature = "io_safety" , since = "1.63.0")] pub struct OwnedFd { fd : ValidRawFd , }
};
}
