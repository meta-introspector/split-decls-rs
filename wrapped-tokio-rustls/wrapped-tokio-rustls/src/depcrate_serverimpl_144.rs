// Generated macro for impl_144 (impl)
macro_rules! Depcrate_serverimpl_144 {
() => {
// Module: crate::server
// Provides: {"impl_144"}
// Dependencies: {}
# [cfg (unix)] impl < IO > AsRawFd for TlsStream < IO > where IO : AsRawFd , { fn as_raw_fd (& self) -> RawFd { self . get_ref () . 0 . as_raw_fd () } }
};
}
