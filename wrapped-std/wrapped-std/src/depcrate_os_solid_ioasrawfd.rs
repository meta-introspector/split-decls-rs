// Generated macro for AsRawFd (trait)
macro_rules! Depcrate_os_solid_ioAsRawFd {
() => {
// Module: crate::os::solid::io
// Provides: {"AsRawFd"}
// Dependencies: {}
# [doc = " A trait to extract the raw SOLID Sockets file descriptor from an underlying"] # [doc = " object."] pub trait AsRawFd { # [doc = " Extracts the raw file descriptor."] # [doc = ""] # [doc = " This method does **not** pass ownership of the raw file descriptor"] # [doc = " to the caller. The descriptor is only guaranteed to be valid while"] # [doc = " the original object has not yet been destroyed."] fn as_raw_fd (& self) -> RawFd ; }
};
}
