// Generated macro for impl_240 (impl)
macro_rules! Depcrate_debugimpl_240 {
() => {
// Module: crate::debug
// Provides: {"impl_240"}
// Dependencies: {}
# [cfg (feature = "system")] impl std :: fmt :: Debug for crate :: Cpu { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Cpu") . field ("name" , & self . name ()) . field ("CPU usage" , & self . cpu_usage ()) . field ("frequency" , & self . frequency ()) . field ("vendor ID" , & self . vendor_id ()) . field ("brand" , & self . brand ()) . finish () } }
};
}
