// Generated macro for impl_31 (impl)
macro_rules! Depcrate_service_spanimpl_31 {
() => {
// Module: crate::service_span
// Provides: {"impl_31"}
// Dependencies: {}
impl < S > Clone for Service < S > where S : Clone , { fn clone (& self) -> Self { Service { span : self . span . clone () , inner : self . inner . clone () , } } }
};
}
