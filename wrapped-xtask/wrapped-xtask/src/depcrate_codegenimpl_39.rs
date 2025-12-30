// Generated macro for impl_39 (impl)
macro_rules! Depcrate_codegenimpl_39 {
() => {
// Module: crate::codegen
// Provides: {"impl_39"}
// Dependencies: {}
impl flags :: Codegen { pub (crate) fn run (self , _sh : & Shell) -> anyhow :: Result < () > { match self . codegen_type . unwrap_or_default () { flags :: CodegenType :: All => { grammar :: generate (self . check) ; assists_doc_tests :: generate (self . check) ; parser_inline_tests :: generate (self . check) ; feature_docs :: generate (self . check) ; diagnostics_docs :: generate (self . check) ; } flags :: CodegenType :: Grammar => grammar :: generate (self . check) , flags :: CodegenType :: AssistsDocTests => assists_doc_tests :: generate (self . check) , flags :: CodegenType :: DiagnosticsDocs => diagnostics_docs :: generate (self . check) , flags :: CodegenType :: LintDefinitions => lints :: generate (self . check) , flags :: CodegenType :: ParserTests => parser_inline_tests :: generate (self . check) , flags :: CodegenType :: FeatureDocs => feature_docs :: generate (self . check) , } Ok (()) } }
};
}
