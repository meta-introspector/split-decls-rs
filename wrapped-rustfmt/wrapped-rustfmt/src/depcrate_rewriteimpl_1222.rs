// Generated macro for impl_1222 (impl)
macro_rules! Depcrate_rewriteimpl_1222 {
() => {
// Module: crate::rewrite
// Provides: {"impl_1222"}
// Dependencies: {}
impl std :: fmt :: Display for MacroErrorKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { MacroErrorKind :: ParseFailure => write ! (f , "(parse failure)") , MacroErrorKind :: ReplaceMacroVariable => write ! (f , "(replacing macro variables with $)") , MacroErrorKind :: Unknown => write ! (f , "") , } } }
};
}
