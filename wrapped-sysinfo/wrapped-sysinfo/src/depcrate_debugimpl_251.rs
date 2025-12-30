// Generated macro for impl_251 (impl)
macro_rules! Depcrate_debugimpl_251 {
() => {
// Module: crate::debug
// Provides: {"impl_251"}
// Dependencies: {}
# [cfg (feature = "user")] impl std :: fmt :: Debug for crate :: Users { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
