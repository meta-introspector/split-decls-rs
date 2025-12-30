// Generated macro for impl_124 (impl)
macro_rules! Depcrate_errorimpl_124 {
() => {
// Module: crate::error
// Provides: {"impl_124"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Io (err) => write ! (f , "{err}") , Error :: Read (err) => write ! (f , "{err}") , Error :: Parse (err) => write ! (f , "{err}") , } } }
};
}
