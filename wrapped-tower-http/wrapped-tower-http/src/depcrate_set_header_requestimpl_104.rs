// Generated macro for impl_104 (impl)
macro_rules! Depcrate_set_header_requestimpl_104 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_104"}
// Dependencies: {}
impl < S , M > fmt :: Debug for SetRequestHeader < S , M > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SetRequestHeader") . field ("inner" , & self . inner) . field ("header_name" , & self . header_name) . field ("mode" , & self . mode) . field ("make" , & std :: any :: type_name :: < M > ()) . finish () } }
};
}
