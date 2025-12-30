// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_services_redirectimpl_1081 {
() => {
// Module: crate::services::redirect
// Provides: {"impl_1081"}
// Dependencies: {}
impl < ResBody > Clone for Redirect < ResBody > { fn clone (& self) -> Self { Self { status_code : self . status_code , location : self . location . clone () , _marker : PhantomData , } } }
};
}
