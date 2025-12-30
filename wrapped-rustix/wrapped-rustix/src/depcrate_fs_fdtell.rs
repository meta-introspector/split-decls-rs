// Generated macro for tell (function)
macro_rules! Depcrate_fs_fdtell {
() => {
// Module: crate::fs::fd
// Provides: {"tell"}
// Dependencies: {}
# [doc = " `lseek(fd, 0, SEEK_CUR)`—Returns the current position within a file."] # [doc = ""] # [doc = " Return the current position of the file descriptor. This is a subset of"] # [doc = " the functionality of `seek`, but this interface makes it easier for users"] # [doc = " to declare their intent not to mutate any state."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/lseek.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html"] # [inline] # [doc (alias = "lseek")] pub fn tell < Fd : AsFd > (fd : Fd) -> io :: Result < u64 > { backend :: fs :: syscalls :: tell (fd . as_fd ()) }
};
}
