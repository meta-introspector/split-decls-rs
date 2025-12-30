// Generated macro for impl_120 (impl)
macro_rules! Depcrate_set_header_responseimpl_120 {
() => {
// Module: crate::set_header::response
// Provides: {"impl_120"}
// Dependencies: {}
impl < S , M > fmt :: Debug for SetResponseHeader < S , M > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SetResponseHeader") . field ("inner" , & self . inner) . field ("header_name" , & self . header_name) . field ("mode" , & self . mode) . field ("make" , & std :: any :: type_name :: < M > ()) . finish () } }
};
}
