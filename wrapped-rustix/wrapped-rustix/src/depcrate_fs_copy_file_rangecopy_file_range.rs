// Generated macro for copy_file_range (function)
macro_rules! Depcrate_fs_copy_file_rangecopy_file_range {
() => {
// Module: crate::fs::copy_file_range
// Provides: {"copy_file_range"}
// Dependencies: {}
# [doc = " `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`—Copies data"] # [doc = " from one file to another."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/copy_file_range.2.html"] # [inline] pub fn copy_file_range < InFd : AsFd , OutFd : AsFd > (fd_in : InFd , off_in : Option < & mut u64 > , fd_out : OutFd , off_out : Option < & mut u64 > , len : usize ,) -> io :: Result < usize > { backend :: fs :: syscalls :: copy_file_range (fd_in . as_fd () , off_in , fd_out . as_fd () , off_out , len) }
};
}
