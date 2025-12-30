// Generated macro for fadvise (function)
macro_rules! Depcrate_fs_fadvisefadvise {
() => {
// Module: crate::fs::fadvise
// Provides: {"fadvise"}
// Dependencies: {}
# [doc = " `posix_fadvise(fd, offset, len, advice)`—Declares an expected access"] # [doc = " pattern for a file."] # [doc = ""] # [doc = " If `len` is `None`, the advice extends to the end of the file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/posix_fadvise.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/posix_fadvise.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=posix_fadvise&sektion=2"] # [inline] # [doc (alias = "posix_fadvise")] pub fn fadvise < Fd : AsFd > (fd : Fd , offset : u64 , len : Option < NonZeroU64 > , advice : Advice ,) -> io :: Result < () > { backend :: fs :: syscalls :: fadvise (fd . as_fd () , offset , len , advice) }
};
}
