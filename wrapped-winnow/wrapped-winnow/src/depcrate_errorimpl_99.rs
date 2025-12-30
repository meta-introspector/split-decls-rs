// Generated macro for impl_99 (impl)
macro_rules! Depcrate_errorimpl_99 {
() => {
// Module: crate::error
// Provides: {"impl_99"}
// Dependencies: {}
impl < C : Clone > Clone for ContextError < C > { fn clone (& self) -> Self { Self { context : self . context . clone () , # [cfg (feature = "std")] cause : self . cause . as_ref () . map (| e | e . to_string () . into ()) , } } }
};
}
