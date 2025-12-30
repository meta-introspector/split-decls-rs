// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (windows)] impl < S > AsRawSocket for TlsStream < S > where S : AsRawSocket , { fn as_raw_socket (& self) -> RawSocket { self . get_ref () . get_ref () . get_ref () . as_raw_socket () } }
};
}
