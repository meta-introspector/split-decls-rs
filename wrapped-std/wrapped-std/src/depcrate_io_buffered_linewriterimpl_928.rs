// Generated macro for impl_928 (impl)
macro_rules! Depcrate_io_buffered_linewriterimpl_928 {
() => {
// Module: crate::io::buffered::linewriter
// Provides: {"impl_928"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > fmt :: Debug for LineWriter < W > where W : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("LineWriter") . field ("writer" , & self . get_ref ()) . field ("buffer" , & format_args ! ("{}/{}" , self . inner . buffer () . len () , self . inner . capacity ()) ,) . finish_non_exhaustive () } }
};
}
