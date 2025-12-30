// Generated macro for impl_249 (impl)
macro_rules! Depcrate_debugimpl_249 {
() => {
// Module: crate::debug
// Provides: {"impl_249"}
// Dependencies: {}
# [cfg (feature = "network")] impl std :: fmt :: Debug for crate :: Networks { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
