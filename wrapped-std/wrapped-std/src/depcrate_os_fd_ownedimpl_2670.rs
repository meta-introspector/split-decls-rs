// Generated macro for impl_2670 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2670 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2670"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl Drop for OwnedFd { # [inline] fn drop (& mut self) { unsafe { # [cfg (not (target_os = "hermit"))] { # [cfg (unix)] crate :: sys :: fs :: debug_assert_fd_is_open (self . fd . as_inner ()) ; let _ = libc :: close (self . fd . as_inner ()) ; } # [cfg (target_os = "hermit")] let _ = hermit_abi :: close (self . fd . as_inner ()) ; } } }
};
}
