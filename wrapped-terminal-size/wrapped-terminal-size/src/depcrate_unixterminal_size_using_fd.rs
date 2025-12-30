// Generated macro for terminal_size_using_fd (function)
macro_rules! Depcrate_unixterminal_size_using_fd {
() => {
// Module: crate::unix
// Provides: {"terminal_size_using_fd"}
// Dependencies: {}
# [doc = " Returns the size of the terminal using the given raw file descriptor, if available."] # [doc = ""] # [doc = " The given file descriptor must be an open file descriptor."] # [doc = ""] # [doc = " If the given file descriptor is not a tty, returns `None`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `fd` must be a valid open file descriptor."] # [deprecated (note = "Use `terminal_size_of` instead.
     Use `BorrowedFd::borrow_raw` to convert a raw fd into a `BorrowedFd` if needed.")] pub unsafe fn terminal_size_using_fd (fd : RawFd) -> Option < (Width , Height) > { terminal_size_of (BorrowedFd :: borrow_raw (fd)) }
};
}
