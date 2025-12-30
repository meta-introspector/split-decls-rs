// Generated macro for TryIntoRawFd (trait)
macro_rules! Depcrate_os_fortanix_sgx_ioTryIntoRawFd {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"TryIntoRawFd"}
// Dependencies: {}
# [doc = " A trait to express the ability to consume an object and acquire ownership of"] # [doc = " its raw file descriptor."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub trait TryIntoRawFd : Sized { # [doc = " Consumes this object, returning the raw underlying file descriptor, if"] # [doc = " this object is not cloned."] # [doc = ""] # [doc = " This function **transfers ownership** of the underlying file descriptor"] # [doc = " to the caller. Callers are then the unique owners of the file descriptor"] # [doc = " and must close the descriptor once it's no longer needed."] # [doc = ""] # [doc = " Unlike other platforms, on SGX, the file descriptor is shared between"] # [doc = " all clones of an object. To avoid race conditions, this function will"] # [doc = " only return `Ok` when called on the final clone."] # [unstable (feature = "sgx_platform" , issue = "56975")] fn try_into_raw_fd (self) -> Result < RawFd , Self > ; }
};
}
