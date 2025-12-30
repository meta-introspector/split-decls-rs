// Generated macro for impl_as_raw_fd (macro)
macro_rules! Depcrate_os_fd_netimpl_as_raw_fd {
() => {
// Module: crate::os::fd::net
// Provides: {"impl_as_raw_fd"}
// Dependencies: {}
macro_rules ! impl_as_raw_fd { ($ ($ t : ident) *) => { $ (# [stable (feature = "rust1" , since = "1.0.0")] impl AsRawFd for net ::$ t { # [inline] fn as_raw_fd (& self) -> RawFd { self . as_inner () . socket () . as_raw_fd () } }) * } ; }
};
}
