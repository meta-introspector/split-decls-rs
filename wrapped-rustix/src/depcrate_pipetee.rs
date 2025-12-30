// Generated macro for tee (function)
macro_rules! Depcrate_pipetee {
() => {
// Module: crate::pipe
// Provides: {"tee"}
// Dependencies: {}
# [doc = " `tee(fd_in, fd_out, len, flags)`—Copy data between pipes without"] # [doc = " consuming it."] # [doc = ""] # [doc = " This reads up to `len` bytes from `in_fd` without consuming them, and"] # [doc = " writes them to `out_fd`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/tee.2.html"] # [cfg (linux_kernel)] # [inline] pub fn tee < FdIn : AsFd , FdOut : AsFd > (fd_in : FdIn , fd_out : FdOut , len : usize , flags : SpliceFlags ,) -> io :: Result < usize > { backend :: pipe :: syscalls :: tee (fd_in . as_fd () , fd_out . as_fd () , len , flags) }
};
}
