// Generated macro for impl_252 (impl)
macro_rules! Depcrate_debugimpl_252 {
() => {
// Module: crate::debug
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (feature = "user")] impl std :: fmt :: Debug for crate :: User { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("User") . field ("uid" , & self . id ()) . field ("gid" , & self . group_id ()) . field ("name" , & self . name ()) . finish () } }
};
}
