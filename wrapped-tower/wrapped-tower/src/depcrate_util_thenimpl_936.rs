// Generated macro for impl_936 (impl)
macro_rules! Depcrate_util_thenimpl_936 {
() => {
// Module: crate::util::then
// Provides: {"impl_936"}
// Dependencies: {}
impl < S , F > fmt :: Debug for Then < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Then") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
