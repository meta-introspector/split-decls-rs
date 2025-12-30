// Generated macro for IntoRawFd (trait)
macro_rules! Depcrate_os_fd_rawIntoRawFd {
() => {
// Module: crate::os::fd::raw
// Provides: {"IntoRawFd"}
// Dependencies: {}
# [doc = " A trait to express the ability to consume an object and acquire ownership of"] # [doc = " its raw file descriptor."] # [stable (feature = "into_raw_os" , since = "1.4.0")] pub trait IntoRawFd { # [doc = " Consumes this object, returning the raw underlying file descriptor."] # [doc = ""] # [doc = " This function is typically used to **transfer ownership** of the underlying"] # [doc = " file descriptor to the caller. When used in this way, callers are then the unique"] # [doc = " owners of the file descriptor and must close it once it's no longer needed."] # [doc = ""] # [doc = " However, transferring ownership is not strictly required. Use a"] # [doc = " [`Into<OwnedFd>::into`] implementation for an API which strictly"] # [doc = " transfers ownership."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs::File;"] # [doc = " # use std::io;"] # [doc = " #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " use std::os::fd::{IntoRawFd, RawFd};"] # [doc = ""] # [doc = " let f = File::open(\"foo.txt\")?;"] # [doc = " #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " let raw_fd: RawFd = f.into_raw_fd();"] # [doc = " # Ok::<(), io::Error>(())"] # [doc = " ```"] # [must_use = "losing the raw file descriptor may leak resources"] # [stable (feature = "into_raw_os" , since = "1.4.0")] fn into_raw_fd (self) -> RawFd ; }
};
}
