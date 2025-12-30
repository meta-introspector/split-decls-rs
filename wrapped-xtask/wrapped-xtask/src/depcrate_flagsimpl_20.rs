// Generated macro for impl_20 (impl)
macro_rules! Depcrate_flagsimpl_20 {
() => {
// Module: crate::flags
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Display for CodegenType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: All => write ! (f , "all") , Self :: Grammar => write ! (f , "grammar") , Self :: AssistsDocTests => write ! (f , "assists-doc-tests") , Self :: DiagnosticsDocs => write ! (f , "diagnostics-docs") , Self :: LintDefinitions => write ! (f , "lint-definitions") , Self :: ParserTests => write ! (f , "parser-tests") , Self :: FeatureDocs => write ! (f , "feature-docs") , } } }
};
}
