// Generated macro for impl_50 (impl)
macro_rules! Depcrate_layerimpl_50 {
() => {
// Module: crate::layer
// Provides: {"impl_50"}
// Dependencies: {}
impl < S , F : fmt :: Debug > fmt :: Debug for ErrorLayer < S , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ErrorLayer") . field ("format" , & self . format) . field ("subscriber" , & format_args ! ("{}" , type_name ::< S > ())) . finish () } }
};
}
