// Generated macro for getpath (function)
macro_rules! Depcrate_fs_getpathgetpath {
() => {
// Module: crate::fs::getpath
// Provides: {"getpath"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_GETPATH)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [inline] pub fn getpath < Fd : AsFd > (fd : Fd) -> io :: Result < CString > { backend :: fs :: syscalls :: getpath (fd . as_fd ()) }
};
}
