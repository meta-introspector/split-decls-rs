// Generated macro for impl_858 (impl)
macro_rules! Depcrate_util_map_resultimpl_858 {
() => {
// Module: crate::util::map_result
// Provides: {"impl_858"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapResult < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapResult") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
