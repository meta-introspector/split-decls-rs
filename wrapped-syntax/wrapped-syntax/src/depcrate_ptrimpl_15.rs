// Generated macro for impl_15 (impl)
macro_rules! Depcrate_ptrimpl_15 {
() => {
// Module: crate::ptr
// Provides: {"impl_15"}
// Dependencies: {}
impl < N : AstNode > std :: fmt :: Debug for AstPtr < N > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("AstPtr") . field (& self . raw) . finish () } }
};
}
