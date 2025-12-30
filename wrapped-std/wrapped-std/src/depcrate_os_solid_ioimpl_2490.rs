// Generated macro for impl_2490 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2490 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2490"}
// Dependencies: {}
impl FromRawFd for OwnedFd { # [doc = " Constructs a new instance of `Self` from the given raw file descriptor."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `fd` must be open and suitable for assuming"] # [doc = " ownership. The resource must not require any cleanup other than `close`."] # [inline] # [track_caller] unsafe fn from_raw_fd (fd : RawFd) -> Self { Self { fd : ValidRawFd :: new (fd) . expect ("fd != -1") } } }
};
}
