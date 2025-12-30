// Generated macro for impl_98 (impl)
macro_rules! Depcrate_set_header_requestimpl_98 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_98"}
// Dependencies: {}
impl < M > fmt :: Debug for SetRequestHeaderLayer < M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SetRequestHeaderLayer") . field ("header_name" , & self . header_name) . field ("mode" , & self . mode) . field ("make" , & std :: any :: type_name :: < M > ()) . finish () } }
};
}
