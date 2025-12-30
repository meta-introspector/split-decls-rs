// Generated macro for userfaultfd (function)
macro_rules! Depcrate_mm_userfaultfduserfaultfd {
() => {
// Module: crate::mm::userfaultfd
// Provides: {"userfaultfd"}
// Dependencies: {}
# [doc = " `userfaultfd(flags)`—Create userspace page-fault handler."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The call itself is safe, but the returned file descriptor lets users"] # [doc = " observe and manipulate process memory in magical ways."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Linux userfaultfd]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/userfaultfd.2.html"] # [doc = " [Linux userfaultfd]: https://www.kernel.org/doc/Documentation/vm/userfaultfd.txt"] # [inline] pub unsafe fn userfaultfd (flags : UserfaultfdFlags) -> io :: Result < OwnedFd > { backend :: mm :: syscalls :: userfaultfd (flags) }
};
}
