// Generated macro for impl_34 (impl)
macro_rules! Depcrate_clientimpl_34 {
() => {
// Module: crate::client
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (windows)] impl < S > AsRawSocket for TlsStream < S > where S : AsRawSocket , { fn as_raw_socket (& self) -> RawSocket { self . get_ref () . 0 . as_raw_socket () } }
};
}
