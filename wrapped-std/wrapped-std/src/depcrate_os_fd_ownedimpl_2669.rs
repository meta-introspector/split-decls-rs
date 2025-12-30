// Generated macro for impl_2669 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2669 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2669"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl FromRawFd for OwnedFd { # [doc = " Constructs a new instance of `Self` from the given raw file descriptor."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `fd` must be open and suitable for assuming"] # [doc = " [ownership][io-safety]. The resource must not require any cleanup other than `close`."] # [doc = ""] # [doc = " [io-safety]: io#io-safety"] # [inline] # [track_caller] unsafe fn from_raw_fd (fd : RawFd) -> Self { Self { fd : ValidRawFd :: new (fd) . expect ("fd != -1") } } }
};
}
