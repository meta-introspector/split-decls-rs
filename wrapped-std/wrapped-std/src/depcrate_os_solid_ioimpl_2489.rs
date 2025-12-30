// Generated macro for impl_2489 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2489 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2489"}
// Dependencies: {}
impl IntoRawFd for OwnedFd { # [inline] fn into_raw_fd (self) -> RawFd { ManuallyDrop :: new (self) . fd . as_inner () } }
};
}
