// Generated macro for impl_246 (impl)
macro_rules! Depcrate_debugimpl_246 {
() => {
// Module: crate::debug
// Provides: {"impl_246"}
// Dependencies: {}
# [cfg (feature = "disk")] impl std :: fmt :: Debug for crate :: Disks { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
