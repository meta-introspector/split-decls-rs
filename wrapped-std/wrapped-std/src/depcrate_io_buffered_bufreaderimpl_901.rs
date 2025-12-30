// Generated macro for impl_901 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_901 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_901"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < R > fmt :: Debug for BufReader < R > where R : ? Sized + fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BufReader") . field ("reader" , & & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . filled () - self . buf . pos () , self . capacity ()) ,) . finish () } }
};
}
