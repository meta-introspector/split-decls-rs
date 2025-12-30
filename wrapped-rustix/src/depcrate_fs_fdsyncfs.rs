// Generated macro for syncfs (function)
macro_rules! Depcrate_fs_fdsyncfs {
() => {
// Module: crate::fs::fd
// Provides: {"syncfs"}
// Dependencies: {}
# [doc = " `syncfs(fd)`—Flush cached filesystem data."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/syncfs.2.html"] # [cfg (linux_kernel)] # [inline] pub fn syncfs < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: syncfs (fd . as_fd ()) }
};
}
