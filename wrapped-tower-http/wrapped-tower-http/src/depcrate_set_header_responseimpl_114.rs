// Generated macro for impl_114 (impl)
macro_rules! Depcrate_set_header_responseimpl_114 {
() => {
// Module: crate::set_header::response
// Provides: {"impl_114"}
// Dependencies: {}
impl < M > fmt :: Debug for SetResponseHeaderLayer < M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SetResponseHeaderLayer") . field ("header_name" , & self . header_name) . field ("mode" , & self . mode) . field ("make" , & std :: any :: type_name :: < M > ()) . finish () } }
};
}
