// Generated macro for ForeignRawFd (type)
macro_rules! Depcrate_process_pidfd_getfdForeignRawFd {
() => {
// Module: crate::process::pidfd_getfd
// Provides: {"ForeignRawFd"}
// Dependencies: {}
# [doc = " Raw file descriptor in another process."] # [doc = ""] # [doc = " A distinct type alias is used here to inform the user that normal file"] # [doc = " descriptors from the calling process should not be used. The provided file"] # [doc = " descriptor is used by the kernel as the index into the file descriptor"] # [doc = " table of an entirely different process."] pub type ForeignRawFd = RawFd ;
};
}
