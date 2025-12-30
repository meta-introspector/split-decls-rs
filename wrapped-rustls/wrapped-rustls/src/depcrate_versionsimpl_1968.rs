// Generated macro for impl_1968 (impl)
macro_rules! Depcrate_versionsimpl_1968 {
() => {
// Module: crate::versions
// Provides: {"impl_1968"}
// Dependencies: {}
impl SupportedProtocolVersion { # [doc = " The TLS enumeration naming this version."] pub const fn version (& self) -> ProtocolVersion { match self { Self :: TLS12 (_) => ProtocolVersion :: TLSv1_2 , Self :: TLS13 (_) => ProtocolVersion :: TLSv1_3 , } } }
};
}
