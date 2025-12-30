// Generated macro for AsFd (trait)
macro_rules! Depcrate_os_fd_ownedAsFd {
() => {
// Module: crate::os::fd::owned
// Provides: {"AsFd"}
// Dependencies: {}
# [doc = " A trait to borrow the file descriptor from an underlying object."] # [doc = ""] # [doc = " This is only available on unix platforms and must be imported in order to"] # [doc = " call the method. Windows platforms have a corresponding `AsHandle` and"] # [doc = " `AsSocket` set of traits."] # [stable (feature = "io_safety" , since = "1.63.0")] pub trait AsFd { # [doc = " Borrows the file descriptor."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs::File;"] # [doc = " # use std::io;"] # [doc = " # #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " # use std::os::fd::{AsFd, BorrowedFd};"] # [doc = ""] # [doc = " let mut f = File::open(\"foo.txt\")?;"] # [doc = " # #[cfg(any(unix, target_os = \"wasi\"))]"] # [doc = " let borrowed_fd: BorrowedFd<'_> = f.as_fd();"] # [doc = " # Ok::<(), io::Error>(())"] # [doc = " ```"] # [stable (feature = "io_safety" , since = "1.63.0")] fn as_fd (& self) -> BorrowedFd < '_ > ; }
};
}
