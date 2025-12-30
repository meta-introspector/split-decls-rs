// Generated macro for impl_29 (impl)
macro_rules! Depcrate_errorimpl_29 {
() => {
// Module: crate::error
// Provides: {"impl_29"}
// Dependencies: {}
impl fmt :: Display for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: CreateFile { path , .. } => { write ! (f , "cannot create output file. path={}" , path . display ()) } Self :: FlushFile { .. } => write ! (f , "cannot flush output buffer") , } } }
};
}
