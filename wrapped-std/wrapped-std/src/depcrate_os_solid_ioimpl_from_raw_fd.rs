// Generated macro for impl_from_raw_fd (macro)
macro_rules! Depcrate_os_solid_ioimpl_from_raw_fd {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_from_raw_fd"}
// Dependencies: {}
macro_rules ! impl_from_raw_fd { ($ ($ t : ident) *) => { $ (# [stable (feature = "from_raw_os" , since = "1.1.0")] impl FromRawFd for net ::$ t { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> net ::$ t { let socket = unsafe { sys :: net :: Socket :: from_raw_fd (fd) } ; net ::$ t :: from_inner (sys :: net ::$ t :: from_inner (socket)) } }) * } ; }
};
}
