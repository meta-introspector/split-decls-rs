// Generated macro for impl_2505 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2505 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2505"}
// Dependencies: {}
impl < T : AsFd > AsFd for Box < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
