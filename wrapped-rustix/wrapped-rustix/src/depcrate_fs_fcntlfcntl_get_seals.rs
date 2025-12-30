// Generated macro for fcntl_get_seals (function)
macro_rules! Depcrate_fs_fcntlfcntl_get_seals {
() => {
// Module: crate::fs::fcntl
// Provides: {"fcntl_get_seals"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_GET_SEALS)`—Return the seals for `fd`'s inode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia"))] # [inline] # [doc (alias = "F_GET_SEALS")] pub fn fcntl_get_seals < Fd : AsFd > (fd : Fd) -> io :: Result < SealFlags > { backend :: fs :: syscalls :: fcntl_get_seals (fd . as_fd ()) }
};
}
