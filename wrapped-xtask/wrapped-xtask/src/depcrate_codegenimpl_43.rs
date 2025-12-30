// Generated macro for impl_43 (impl)
macro_rules! Depcrate_codegenimpl_43 {
() => {
// Module: crate::codegen
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Display for Location { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let path = self . file . strip_prefix (project_root ()) . unwrap () . display () . to_string () ; let path = path . replace ('\\' , "/") ; let name = self . file . file_name () . unwrap () ; write ! (f , " [{}](https://github.com/rust-lang/rust-analyzer/blob/master/{}#L{}) " , name . to_str () . unwrap () , path , self . line) } }
};
}
