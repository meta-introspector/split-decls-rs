// Generated macro for impl_243 (impl)
macro_rules! Depcrate_debugimpl_243 {
() => {
// Module: crate::debug
// Provides: {"impl_243"}
// Dependencies: {}
# [cfg (feature = "system")] impl std :: fmt :: Debug for crate :: Product { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Product") . field ("name" , & Self :: name ()) . field ("family" , & Self :: family ()) . field ("serial_number" , & Self :: serial_number ()) . field ("stock_keeping_unit" , & Self :: stock_keeping_unit ()) . field ("uuid" , & Self :: uuid ()) . field ("version" , & Self :: version ()) . field ("vendor_name" , & Self :: vendor_name ()) . finish () } }
};
}
