// Generated macro for fcntl_add_seals (function)
macro_rules! Depcrate_fs_fcntlfcntl_add_seals {
() => {
// Module: crate::fs::fcntl
// Provides: {"fcntl_add_seals"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_ADD_SEALS)`—Add seals to `fd`'s inode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia"))] # [inline] # [doc (alias = "F_ADD_SEALS")] pub fn fcntl_add_seals < Fd : AsFd > (fd : Fd , seals : SealFlags) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_add_seals (fd . as_fd () , seals) }
};
}
