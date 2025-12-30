// Generated macro for AsRawFd (trait)
macro_rules! Depcrate_os_fortanix_sgx_ioAsRawFd {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"AsRawFd"}
// Dependencies: {}
# [doc = " A trait to extract the raw SGX file descriptor from an underlying"] # [doc = " object."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub trait AsRawFd { # [doc = " Extracts the raw file descriptor."] # [doc = ""] # [doc = " This method does **not** pass ownership of the raw file descriptor"] # [doc = " to the caller. The descriptor is only guaranteed to be valid while"] # [doc = " the original object has not yet been destroyed."] # [unstable (feature = "sgx_platform" , issue = "56975")] fn as_raw_fd (& self) -> RawFd ; }
};
}
