// Generated macro for impl_19 (impl)
macro_rules! Depcrate_mapimpl_19 {
() => {
// Module: crate::map
// Provides: {"impl_19"}
// Dependencies: {}
impl < K : Clone , V : Clone > Clone for Map < K , V > { # [inline] fn clone (& self) -> Self { Self { map : self . map . clone () , dotted : self . dotted , implicit : self . implicit , inline : self . inline , } } }
};
}
