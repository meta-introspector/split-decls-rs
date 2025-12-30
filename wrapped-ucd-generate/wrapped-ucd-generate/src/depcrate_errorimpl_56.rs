// Generated macro for impl_56 (impl)
macro_rules! Depcrate_errorimpl_56 {
() => {
// Module: crate::error
// Provides: {"impl_56"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: Io (ref err) => err . fmt (f) , Error :: Clap (ref err) => err . fmt (f) , Error :: Other (ref msg) => write ! (f , "{}" , msg) , } } }
};
}
