// Generated macro for impl_37 (impl)
macro_rules! Depcrate_errorimpl_37 {
() => {
// Module: crate::error
// Provides: {"impl_37"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . inner { ErrorInner :: Io { path : None , ref err } => err . fmt (f) , ErrorInner :: Io { path : Some (ref path) , ref err } => write ! (f , "IO error for operation on {}: {}" , path . display () , err) , ErrorInner :: Loop { ref ancestor , ref child } => write ! (f , "File system loop found: \
                 {} points to an ancestor {}" , child . display () , ancestor . display ()) , } } }
};
}
