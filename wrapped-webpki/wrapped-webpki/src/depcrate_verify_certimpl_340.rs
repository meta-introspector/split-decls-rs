// Generated macro for impl_340 (impl)
macro_rules! Depcrate_verify_certimpl_340 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_340"}
// Dependencies: {}
impl EkuValidationMode { fn id (& self) -> KeyPurposeId < 'static > { match self { Self :: Required (id) => * id , Self :: RequiredIfPresent (id) => * id , } } }
};
}
