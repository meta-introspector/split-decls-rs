// Generated macro for impl_2500 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2500 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2500"}
// Dependencies: {}
impl AsFd for OwnedFd { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (self . as_raw_fd ()) } } }
};
}
