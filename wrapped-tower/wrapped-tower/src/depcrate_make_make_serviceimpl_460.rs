// Generated macro for impl_460 (impl)
macro_rules! Depcrate_make_make_serviceimpl_460 {
() => {
// Module: crate::make::make_service
// Provides: {"impl_460"}
// Dependencies: {}
impl < M , Request > fmt :: Debug for AsService < '_ , M , Request > where M : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AsService") . field ("make" , & self . make) . finish () } }
};
}
