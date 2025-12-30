// Generated macro for impl_2497 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2497 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2497"}
// Dependencies: {}
impl < T : AsFd > AsFd for & T { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { T :: as_fd (self) } }
};
}
