// Generated macro for impl_100 (impl)
macro_rules! Depcrate_fileimpl_100 {
() => {
// Module: crate::file
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (any (unix , target_os = "wasi"))] impl < F : AsRawFd > AsRawFd for NamedTempFile < F > { # [inline] fn as_raw_fd (& self) -> RawFd { self . as_file () . as_raw_fd () } }
};
}
