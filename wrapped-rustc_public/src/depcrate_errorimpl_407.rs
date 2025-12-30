// Generated macro for impl_407 (impl)
macro_rules! Depcrate_errorimpl_407 {
() => {
// Module: crate::error
// Provides: {"impl_407"}
// Dependencies: {}
impl < T > Debug for CompilerError < T > where T : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason:?}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }
};
}
