// Generated macro for impl_831 (impl)
macro_rules! Depcrate_util_map_requestimpl_831 {
() => {
// Module: crate::util::map_request
// Provides: {"impl_831"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapRequest < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapRequest") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
