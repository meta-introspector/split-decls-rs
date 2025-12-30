// Generated macro for IntoRawFd (trait)
macro_rules! Depcrate_os_solid_ioIntoRawFd {
() => {
// Module: crate::os::solid::io
// Provides: {"IntoRawFd"}
// Dependencies: {}
# [doc = " A trait to express the ability to consume an object and acquire ownership of"] # [doc = " its raw file descriptor."] pub trait IntoRawFd { # [doc = " Consumes this object, returning the raw underlying file descriptor."] # [doc = ""] # [doc = " This function **transfers ownership** of the underlying file descriptor"] # [doc = " to the caller. Callers are then the unique owners of the file descriptor"] # [doc = " and must close the descriptor once it's no longer needed."] # [must_use = "losing the raw file descriptor may leak resources"] fn into_raw_fd (self) -> RawFd ; }
};
}
