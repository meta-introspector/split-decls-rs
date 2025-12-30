// Generated macro for impl_145 (impl)
macro_rules! Depcrate_serverimpl_145 {
() => {
// Module: crate::server
// Provides: {"impl_145"}
// Dependencies: {}
# [cfg (windows)] impl < IO > AsRawSocket for TlsStream < IO > where IO : AsRawSocket , { fn as_raw_socket (& self) -> RawSocket { self . get_ref () . 0 . as_raw_socket () } }
};
}
