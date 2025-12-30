// Generated macro for impl_375 (impl)
macro_rules! Depcrate_config_file_linesimpl_375 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_375"}
// Dependencies: {}
impl fmt :: Display for FileName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FileName :: Real (p) => write ! (f , "{}" , p . display ()) , FileName :: Stdin => write ! (f , "<stdin>") , } } }
};
}
