// Generated macro for impl_86 (impl)
macro_rules! Depcrate_serimpl_86 {
() => {
// Module: crate::ser
// Provides: {"impl_86"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: Custom (ref msg) => msg . fmt (f) , Error :: Utf8 (ref err) => write ! (f , "invalid UTF-8: {}" , err) , } } }
};
}
