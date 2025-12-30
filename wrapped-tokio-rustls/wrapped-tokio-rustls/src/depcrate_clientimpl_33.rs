// Generated macro for impl_33 (impl)
macro_rules! Depcrate_clientimpl_33 {
() => {
// Module: crate::client
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (unix)] impl < S > AsRawFd for TlsStream < S > where S : AsRawFd , { fn as_raw_fd (& self) -> RawFd { self . get_ref () . 0 . as_raw_fd () } }
};
}
