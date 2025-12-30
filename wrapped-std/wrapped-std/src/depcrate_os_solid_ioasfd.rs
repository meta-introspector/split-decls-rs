// Generated macro for AsFd (trait)
macro_rules! Depcrate_os_solid_ioAsFd {
() => {
// Module: crate::os::solid::io
// Provides: {"AsFd"}
// Dependencies: {}
# [doc = " A trait to borrow the SOLID Sockets file descriptor from an underlying"] # [doc = " object."] pub trait AsFd { # [doc = " Borrows the file descriptor."] fn as_fd (& self) -> BorrowedFd < '_ > ; }
};
}
