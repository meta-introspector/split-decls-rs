// Generated macro for impl_2498 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2498 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2498"}
// Dependencies: {}
impl < T : AsFd > AsFd for & mut T { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { T :: as_fd (self) } }
};
}
