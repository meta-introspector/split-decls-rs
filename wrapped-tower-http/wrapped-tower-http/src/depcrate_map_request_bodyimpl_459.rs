// Generated macro for impl_459 (impl)
macro_rules! Depcrate_map_request_bodyimpl_459 {
() => {
// Module: crate::map_request_body
// Provides: {"impl_459"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapRequestBody < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapRequestBody") . field ("inner" , & self . inner) . field ("f" , & std :: any :: type_name :: < F > ()) . finish () } }
};
}
