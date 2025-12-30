// Generated macro for fstatfs (function)
macro_rules! Depcrate_fs_fdfstatfs {
() => {
// Module: crate::fs::fd
// Provides: {"fstatfs"}
// Dependencies: {}
# [doc = " `fstatfs(fd)`—Queries filesystem statistics for an open file or directory."] # [doc = ""] # [doc = " Compared to [`fstatvfs`], this function often provides more information,"] # [doc = " though it's less portable."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fstatfs.2.html"] # [cfg (not (any (solarish , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "netbsd" , target_os = "nto" , target_os = "redox" , target_os = "vita" , target_os = "wasi" ,)))] # [inline] pub fn fstatfs < Fd : AsFd > (fd : Fd) -> io :: Result < StatFs > { backend :: fs :: syscalls :: fstatfs (fd . as_fd ()) }
};
}
