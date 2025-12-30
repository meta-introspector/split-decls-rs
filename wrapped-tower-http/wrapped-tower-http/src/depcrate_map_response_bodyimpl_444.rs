// Generated macro for impl_444 (impl)
macro_rules! Depcrate_map_response_bodyimpl_444 {
() => {
// Module: crate::map_response_body
// Provides: {"impl_444"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapResponseBody < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapResponseBody") . field ("inner" , & self . inner) . field ("f" , & std :: any :: type_name :: < F > ()) . finish () } }
};
}
