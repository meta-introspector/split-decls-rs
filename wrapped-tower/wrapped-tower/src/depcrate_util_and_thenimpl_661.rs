// Generated macro for impl_661 (impl)
macro_rules! Depcrate_util_and_thenimpl_661 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_661"}
// Dependencies: {}
impl < S , F > fmt :: Debug for AndThen < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AndThen") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
