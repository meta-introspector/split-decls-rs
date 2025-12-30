// Generated macro for impl_2663 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2663 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2663"}
// Dependencies: {}
impl BorrowedFd < '_ > { # [doc = " Returns a `BorrowedFd` holding the given raw file descriptor."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `fd` must remain open for the duration of"] # [doc = " the returned `BorrowedFd`, and it must not have the value `-1`."] # [inline] # [track_caller] # [rustc_const_stable (feature = "io_safety" , since = "1.63.0")] # [stable (feature = "io_safety" , since = "1.63.0")] pub const unsafe fn borrow_raw (fd : RawFd) -> Self { Self { fd : ValidRawFd :: new (fd) . expect ("fd != -1") , _phantom : PhantomData } } }
};
}
