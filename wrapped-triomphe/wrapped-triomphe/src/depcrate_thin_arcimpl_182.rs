// Generated macro for impl_182 (impl)
macro_rules! Depcrate_thin_arcimpl_182 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_182"}
// Dependencies: {}
impl < H , T > Clone for ThinArc < H , T > { # [inline] fn clone (& self) -> Self { ThinArc :: with_protected_arc (self , | a | Arc :: protected_into_thin (a . clone ())) } }
};
}
