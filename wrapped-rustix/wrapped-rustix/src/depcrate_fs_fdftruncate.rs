// Generated macro for ftruncate (function)
macro_rules! Depcrate_fs_fdftruncate {
() => {
// Module: crate::fs::fd
// Provides: {"ftruncate"}
// Dependencies: {}
# [doc = " `ftruncate(fd, length)`—Sets the length of a file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/ftruncate.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/ftruncate.2.html"] # [inline] pub fn ftruncate < Fd : AsFd > (fd : Fd , length : u64) -> io :: Result < () > { backend :: fs :: syscalls :: ftruncate (fd . as_fd () , length) }
};
}
