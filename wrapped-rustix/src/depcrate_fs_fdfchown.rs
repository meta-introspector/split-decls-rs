// Generated macro for fchown (function)
macro_rules! Depcrate_fs_fdfchown {
() => {
// Module: crate::fs::fd
// Provides: {"fchown"}
// Dependencies: {}
# [doc = " `fchown(fd, owner, group)`—Sets open file or directory ownership."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchown.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fchown.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn fchown < Fd : AsFd > (fd : Fd , owner : Option < Uid > , group : Option < Gid >) -> io :: Result < () > { backend :: fs :: syscalls :: fchown (fd . as_fd () , owner , group) }
};
}
