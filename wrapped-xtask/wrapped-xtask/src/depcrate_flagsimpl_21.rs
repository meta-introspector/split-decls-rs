// Generated macro for impl_21 (impl)
macro_rules! Depcrate_flagsimpl_21 {
() => {
// Module: crate::flags
// Provides: {"impl_21"}
// Dependencies: {}
impl FromStr for CodegenType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "all" => Ok (Self :: All) , "grammar" => Ok (Self :: Grammar) , "assists-doc-tests" => Ok (Self :: AssistsDocTests) , "diagnostics-docs" => Ok (Self :: DiagnosticsDocs) , "lint-definitions" => Ok (Self :: LintDefinitions) , "parser-tests" => Ok (Self :: ParserTests) , "feature-docs" => Ok (Self :: FeatureDocs) , _ => Err ("Invalid option" . to_owned ()) , } } }
};
}
