// Generated macro for impl_844 (impl)
macro_rules! Depcrate_util_map_responseimpl_844 {
() => {
// Module: crate::util::map_response
// Provides: {"impl_844"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapResponse < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapResponse") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
