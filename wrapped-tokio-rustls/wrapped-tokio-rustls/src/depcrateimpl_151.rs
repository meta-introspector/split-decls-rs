// Generated macro for impl_151 (impl)
macro_rules! Depcrateimpl_151 {
() => {
// Module: crate
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (unix)] impl < S > AsRawFd for TlsStream < S > where S : AsRawFd , { fn as_raw_fd (& self) -> RawFd { self . get_ref () . 0 . as_raw_fd () } }
};
}
