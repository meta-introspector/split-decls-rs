// Generated macro for impl_242 (impl)
macro_rules! Depcrate_debugimpl_242 {
() => {
// Module: crate::debug
// Provides: {"impl_242"}
// Dependencies: {}
# [cfg (feature = "system")] impl std :: fmt :: Debug for crate :: Motherboard { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Motherboard") . field ("name" , & self . name ()) . field ("vendor_name" , & self . vendor_name ()) . field ("version" , & self . version ()) . field ("serial_number" , & self . serial_number ()) . field ("asset_tag" , & self . asset_tag ()) . finish () } }
};
}
