// Generated macro for AsRawFd (trait)
macro_rules! Depcrate_os_fd_rawAsRawFd {
() => {
// Module: crate::os::fd::raw
// Provides: {"AsRawFd"}
// Dependencies: {}
# [doc = " A trait to extract the raw file descriptor from an underlying object."] # [doc = ""] # [doc = " This is only available on unix and WASI platforms and must be imported in"] # [doc = " order to call the method. Windows platforms have a corresponding"] # [doc = " `AsRawHandle` and `AsRawSocket` set of traits."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait AsRawFd { # [doc = " Extracts the raw file descriptor."] # [doc = ""] # [doc = " This function is typically used to **borrow** an owned file descriptor."] # [doc = " When used in this way, this method does **not** pass ownership of the"] # [doc = " raw file descriptor to the caller, and the file descriptor is only"] # [doc = " guaranteed to be valid while the original object has not yet been"] # [doc = " destroyed."] # [doc = ""] # [doc = " However, borrowing is not strictly required. See [`AsFd::as_fd`]"] # [doc = " for an API which strictly borrows a file descriptor."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs::File;"] # [doc = " # use std::io;"] # [doc = " #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " use std::os::fd::{AsRawFd, RawFd};"] # [doc = ""] # [doc = " let mut f = File::open(\"foo.txt\")?;"] # [doc = " // Note that `raw_fd` is only valid as long as `f` exists."] # [doc = " #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " let raw_fd: RawFd = f.as_raw_fd();"] # [doc = " # Ok::<(), io::Error>(())"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] fn as_raw_fd (& self) -> RawFd ; }
};
}
