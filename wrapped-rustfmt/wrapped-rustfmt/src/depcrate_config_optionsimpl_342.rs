// Generated macro for impl_342 (impl)
macro_rules! Depcrate_config_optionsimpl_342 {
() => {
// Module: crate::config::options
// Provides: {"impl_342"}
// Dependencies: {}
impl fmt :: Display for IgnoreList { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "[{}]" , self . path_set . iter () . format_with (", " , | path , f | f (& format_args ! ("{}" , path . to_string_lossy ())))) } }
};
}
