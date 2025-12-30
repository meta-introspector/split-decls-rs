// Generated macro for pidfd_getfd (function)
macro_rules! Depcrate_process_pidfd_getfdpidfd_getfd {
() => {
// Module: crate::process::pidfd_getfd
// Provides: {"pidfd_getfd"}
// Dependencies: {}
# [doc = " `syscall(SYS_pidfd_getfd, pidfd, flags)`—Obtain a duplicate of another"] # [doc = " process' file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is generally safe for the calling process, but it can impact"] # [doc = " the target process in unexpected ways. If you want to ensure that Rust I/O"] # [doc = " safety assumptions continue to hold in the target process, then the target"] # [doc = " process must have communicated the file description number to the calling"] # [doc = " process from a value of a type that implements `AsRawFd`, and the target"] # [doc = " process must not drop that value until after the calling process has"] # [doc = " returned from `pidfd_getfd`."] # [doc = ""] # [doc = " When `pidfd_getfd` is used to debug the target, or the target is not a Rust"] # [doc = " application, or `pidfd_getfd` is used in any other way, then extra care"] # [doc = " should be taken to avoid unexpected behaviour or crashes."] # [doc = ""] # [doc = " For further details, see the references above."] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html"] # [inline] pub fn pidfd_getfd < Fd : AsFd > (pidfd : Fd , targetfd : ForeignRawFd , flags : PidfdGetfdFlags ,) -> io :: Result < OwnedFd > { backend :: process :: syscalls :: pidfd_getfd (pidfd . as_fd () , targetfd , flags) }
};
}
