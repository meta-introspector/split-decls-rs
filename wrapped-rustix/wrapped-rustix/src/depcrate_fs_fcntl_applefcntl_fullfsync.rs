// Generated macro for fcntl_fullfsync (function)
macro_rules! Depcrate_fs_fcntl_applefcntl_fullfsync {
() => {
// Module: crate::fs::fcntl_apple
// Provides: {"fcntl_fullfsync"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_FULLFSYNC)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc (alias = "F_FULLSYNC")] # [inline] pub fn fcntl_fullfsync < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_fullfsync (fd . as_fd ()) }
};
}
