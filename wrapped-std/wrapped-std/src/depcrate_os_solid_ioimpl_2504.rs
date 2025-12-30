// Generated macro for impl_2504 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2504 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2504"}
// Dependencies: {}
impl < T : AsFd > AsFd for crate :: rc :: Rc < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
