// Generated macro for BorrowedFd (struct)
macro_rules! Depcrate_os_fd_ownedBorrowedFd {
() => {
// Module: crate::os::fd::owned
// Provides: {"BorrowedFd"}
// Dependencies: {}
# [doc = " A borrowed file descriptor."] # [doc = ""] # [doc = " This has a lifetime parameter to tie it to the lifetime of something that owns the file"] # [doc = " descriptor. For the duration of that lifetime, it is guaranteed that nobody will close the file"] # [doc = " descriptor."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host file"] # [doc = " descriptor, so it can be used in FFI in places where a file descriptor is"] # [doc = " passed as an argument, it is not captured or consumed, and it never has the"] # [doc = " value `-1`."] # [doc = ""] # [doc = " This type does not have a [`ToOwned`][crate::borrow::ToOwned]"] # [doc = " implementation. Calling `.to_owned()` on a variable of this type will call"] # [doc = " it on `&BorrowedFd` and use `Clone::clone()` like `ToOwned` does for all"] # [doc = " types implementing `Clone`. The result will be descriptor borrowed under"] # [doc = " the same lifetime."] # [doc = ""] # [doc = " To obtain an [`OwnedFd`], you can use [`BorrowedFd::try_clone_to_owned`]"] # [doc = " instead, but this is not supported on all platforms."] # [derive (Copy , Clone)] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] # [stable (feature = "io_safety" , since = "1.63.0")] pub struct BorrowedFd < 'fd > { fd : ValidRawFd , _phantom : PhantomData < & 'fd OwnedFd > , }
};
}
