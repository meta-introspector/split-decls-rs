// Generated macro for impl_2491 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2491 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2491"}
// Dependencies: {}
impl Drop for OwnedFd { # [inline] fn drop (& mut self) { unsafe { crate :: sys :: abi :: sockets :: close (self . fd . as_inner ()) } ; } }
};
}
