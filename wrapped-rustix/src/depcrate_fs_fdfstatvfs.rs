// Generated macro for fstatvfs (function)
macro_rules! Depcrate_fs_fdfstatvfs {
() => {
// Module: crate::fs::fd
// Provides: {"fstatvfs"}
// Dependencies: {}
# [doc = " `fstatvfs(fd)`—Queries filesystem statistics for an open file or"] # [doc = " directory, POSIX version."] # [doc = ""] # [doc = " Compared to [`fstatfs`], this function often provides less information,"] # [doc = " but it is more portable. But even so, filesystems are very diverse and not"] # [doc = " all the fields are meaningful for every filesystem. And `f_fsid` doesn't"] # [doc = " seem to have a clear meaning anywhere."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fstatvfs.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fstatvfs.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn fstatvfs < Fd : AsFd > (fd : Fd) -> io :: Result < StatVfs > { backend :: fs :: syscalls :: fstatvfs (fd . as_fd ()) }
};
}
