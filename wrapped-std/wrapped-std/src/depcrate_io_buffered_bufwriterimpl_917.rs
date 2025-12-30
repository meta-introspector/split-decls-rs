// Generated macro for impl_917 (impl)
macro_rules! Depcrate_io_buffered_bufwriterimpl_917 {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"impl_917"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > fmt :: Debug for BufWriter < W > where W : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BufWriter") . field ("writer" , & & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . finish () } }
};
}
