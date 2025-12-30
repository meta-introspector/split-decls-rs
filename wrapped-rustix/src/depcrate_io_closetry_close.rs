// Generated macro for try_close (function)
macro_rules! Depcrate_io_closetry_close {
() => {
// Module: crate::io::close
// Provides: {"try_close"}
// Dependencies: {}
# [doc = " `close(raw_fd)`—Closes a `RawFd` directly, and report any errors returned"] # [doc = " by the OS."] # [doc = ""] # [doc = " The rustix developers do not intend the existence of this feature to imply"] # [doc = " that anyone should use it."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function takes a `RawFd`, which must be valid before the call, and is"] # [doc = " not valid after the call, even if it fails."] # [cfg (feature = "try_close")] pub unsafe fn try_close (raw_fd : RawFd) -> crate :: io :: Result < () > { backend :: io :: syscalls :: try_close (raw_fd) }
};
}
