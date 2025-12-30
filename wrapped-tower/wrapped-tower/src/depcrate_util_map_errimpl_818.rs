// Generated macro for impl_818 (impl)
macro_rules! Depcrate_util_map_errimpl_818 {
() => {
// Module: crate::util::map_err
// Provides: {"impl_818"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapErr < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapErr") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
