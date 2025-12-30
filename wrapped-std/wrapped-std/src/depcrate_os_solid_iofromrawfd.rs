// Generated macro for FromRawFd (trait)
macro_rules! Depcrate_os_solid_ioFromRawFd {
() => {
// Module: crate::os::solid::io
// Provides: {"FromRawFd"}
// Dependencies: {}
# [doc = " A trait to express the ability to construct an object from a raw file"] # [doc = " descriptor."] pub trait FromRawFd { # [doc = " Constructs a new instance of `Self` from the given raw file"] # [doc = " descriptor."] # [doc = ""] # [doc = " This function is typically used to **consume ownership** of the"] # [doc = " specified file descriptor. When used in this way, the returned object"] # [doc = " will take responsibility for closing it when the object goes out of"] # [doc = " scope."] # [doc = ""] # [doc = " However, consuming ownership is not strictly required. Use a"] # [doc = " [`From<OwnedFd>::from`] implementation for an API which strictly"] # [doc = " consumes ownership."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `fd` passed in must be an [owned file descriptor][io-safety];"] # [doc = " in particular, it must be open."] # [doc = ""] # [doc = " [io-safety]: io#io-safety"] unsafe fn from_raw_fd (fd : RawFd) -> Self ; }
};
}
