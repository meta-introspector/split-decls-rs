// Generated macro for impl_into_raw_fd (macro)
macro_rules! Depcrate_os_fd_netimpl_into_raw_fd {
() => {
// Module: crate::os::fd::net
// Provides: {"impl_into_raw_fd"}
// Dependencies: {}
macro_rules ! impl_into_raw_fd { ($ ($ t : ident) *) => { $ (# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawFd for net ::$ t { # [inline] fn into_raw_fd (self) -> RawFd { self . into_inner () . into_socket () . into_inner () . into_inner () . into_raw_fd () } }) * } ; }
};
}
