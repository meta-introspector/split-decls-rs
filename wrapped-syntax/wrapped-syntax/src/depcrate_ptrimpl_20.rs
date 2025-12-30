// Generated macro for impl_20 (impl)
macro_rules! Depcrate_ptrimpl_20 {
() => {
// Module: crate::ptr
// Provides: {"impl_20"}
// Dependencies: {}
impl < N : AstNode > Hash for AstPtr < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) ; } }
};
}
