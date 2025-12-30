// Generated macro for AsHandle (trait)
macro_rules! Depcrate_os_windows_io_handleAsHandle {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"AsHandle"}
// Dependencies: {}
# [doc = " A trait to borrow the handle from an underlying object."] # [stable (feature = "io_safety" , since = "1.63.0")] pub trait AsHandle { # [doc = " Borrows the handle."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs::File;"] # [doc = " # use std::io;"] # [doc = " use std::os::windows::io::{AsHandle, BorrowedHandle};"] # [doc = ""] # [doc = " let mut f = File::open(\"foo.txt\")?;"] # [doc = " let borrowed_handle: BorrowedHandle<'_> = f.as_handle();"] # [doc = " # Ok::<(), io::Error>(())"] # [doc = " ```"] # [stable (feature = "io_safety" , since = "1.63.0")] fn as_handle (& self) -> BorrowedHandle < '_ > ; }
};
}
