// Generated macro for write (function)
macro_rules! Depcrate_callwrite {
() => {
// Module: crate::call
// Provides: {"write"}
// Dependencies: {}
# [doc = " Write a buffer to a file descriptor"] # [doc = ""] # [doc = " The kernel will attempt to write the bytes in `buf` to the file descriptor `fd`, returning"] # [doc = " either an `Err`, explained below, or `Ok(count)` where `count` is the number of bytes which"] # [doc = " were written."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " * `EAGAIN` - the file descriptor was opened with `O_NONBLOCK` and writing would block"] # [doc = " * `EBADF` - the file descriptor is not valid or is not open for writing"] # [doc = " * `EFAULT` - `buf` does not point to the process's addressible memory"] # [doc = " * `EIO` - an I/O error occurred"] # [doc = " * `ENOSPC` - the device containing the file descriptor has no room for data"] # [doc = " * `EPIPE` - the file descriptor refers to a pipe or socket whose reading end is closed"] pub fn write (fd : usize , buf : & [u8]) -> Result < usize > { unsafe { syscall3 (SYS_WRITE , fd , buf . as_ptr () as usize , buf . len ()) } }
};
}
