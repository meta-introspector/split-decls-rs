// Generated macro for impl_98 (impl)
macro_rules! Depcrate_linked_listimpl_98 {
() => {
// Module: crate::linked_list
// Provides: {"impl_98"}
// Dependencies: {}
impl < T : Clone > Clone for LinkedEntry < T > { # [inline] fn clone (& self) -> Self { Self { instance : self . instance . clone () , next : AtomicShared :: default () , } } }
};
}
