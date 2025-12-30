// Generated macro for impl_174 (impl)
macro_rules! Depcrate_verifying_keyimpl_174 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_174"}
// Dependencies: {}
impl < P : ParameterSet > Clone for VerifyingKey < P > { fn clone (& self) -> Self { VerifyingKey { pk_seed : self . pk_seed . clone () , pk_root : self . pk_root . clone () , } } }
};
}
