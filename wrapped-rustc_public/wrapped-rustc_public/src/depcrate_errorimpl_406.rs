// Generated macro for impl_406 (impl)
macro_rules! Depcrate_errorimpl_406 {
() => {
// Module: crate::error
// Provides: {"impl_406"}
// Dependencies: {}
impl < T > Display for CompilerError < T > where T : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }
};
}
