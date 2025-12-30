// Generated macro for impl_1969 (impl)
macro_rules! Depcrate_versionsimpl_1969 {
() => {
// Module: crate::versions
// Provides: {"impl_1969"}
// Dependencies: {}
impl PartialEq for SupportedProtocolVersion { fn eq (& self , other : & Self) -> bool { matches ! ((self , other) , (Self :: TLS12 (_) , Self :: TLS12 (_)) | (Self :: TLS13 (_) , Self :: TLS13 (_))) } }
};
}
