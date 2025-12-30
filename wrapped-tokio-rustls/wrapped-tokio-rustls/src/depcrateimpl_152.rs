// Generated macro for impl_152 (impl)
macro_rules! Depcrateimpl_152 {
() => {
// Module: crate
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (windows)] impl < S > AsRawSocket for TlsStream < S > where S : AsRawSocket , { fn as_raw_socket (& self) -> RawSocket { self . get_ref () . 0 . as_raw_socket () } }
};
}
