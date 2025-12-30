// Generated macro for ABS (const)
macro_rules! Depcrate_fs_specialABS {
() => {
// Module: crate::fs::special
// Provides: {"ABS"}
// Dependencies: {}
# [doc = " `-EBADF`—A handle that requires paths to be absolute."] # [doc = ""] # [doc = " This is a file descriptor which refers to no directory, which can be used"] # [doc = " as the directory argument in `*at` functions such as [`openat`], which"] # [doc = " causes them to fail with [`BADF`] if the accompanying path is not absolute."] # [doc = ""] # [doc = " This corresponds to the undocumented by commonly used convention of"] # [doc = " passing `-EBADF` as the `dirfd` argument, which is ignored if the path is"] # [doc = " absolute, and evokes an `EBADF` error otherwise."] # [doc = ""] # [doc = " [`openat`]: crate::fs::openat"] # [doc = " [`BADF`]: crate::io::Errno::BADF"] pub const ABS : BorrowedFd < 'static > = unsafe { BorrowedFd :: < 'static > :: borrow_raw (c :: EBADF . wrapping_neg () as RawFd) } ;
};
}
