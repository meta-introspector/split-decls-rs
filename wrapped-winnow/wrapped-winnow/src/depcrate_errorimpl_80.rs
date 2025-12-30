// Generated macro for impl_80 (impl)
macro_rules! Depcrate_errorimpl_80 {
() => {
// Module: crate::error
// Provides: {"impl_80"}
// Dependencies: {}
impl < I : Clone , E > FromExternalError < I , E > for InputError < I > { # [doc = " Create a new error from an input position and an external error"] # [inline] fn from_external_error (input : & I , _e : E) -> Self { Self { input : input . clone () , } } }
};
}
