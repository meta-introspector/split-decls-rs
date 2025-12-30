// Generated macro for impl_247 (impl)
macro_rules! Depcrate_debugimpl_247 {
() => {
// Module: crate::debug
// Provides: {"impl_247"}
// Dependencies: {}
# [cfg (feature = "component")] impl std :: fmt :: Debug for crate :: Components { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
