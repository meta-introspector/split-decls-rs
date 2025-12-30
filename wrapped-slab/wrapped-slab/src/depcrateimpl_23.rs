// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < T > Clone for Slab < T > where T : Clone , { fn clone (& self) -> Self { Self { entries : self . entries . clone () , len : self . len , next : self . next , } } fn clone_from (& mut self , source : & Self) { self . entries . clone_from (& source . entries) ; self . len = source . len ; self . next = source . next ; } }
};
}
