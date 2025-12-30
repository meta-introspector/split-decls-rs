// Generated macro for impl_2484 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2484 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2484"}
// Dependencies: {}
impl BorrowedFd < '_ > { # [doc = " Returns a `BorrowedFd` holding the given raw file descriptor."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `fd` must remain open for the duration of"] # [doc = " the returned `BorrowedFd`, and it must not have the value"] # [doc = " `SOLID_NET_INVALID_FD`."] # [inline] # [track_caller] pub const unsafe fn borrow_raw (fd : RawFd) -> Self { Self { fd : ValidRawFd :: new (fd) . expect ("fd != -1") , _phantom : PhantomData } } }
};
}
