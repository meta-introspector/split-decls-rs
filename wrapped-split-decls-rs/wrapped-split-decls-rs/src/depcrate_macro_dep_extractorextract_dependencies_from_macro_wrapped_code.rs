// Generated macro for extract_dependencies_from_macro_wrapped_code (function)
macro_rules! Depcrate_macro_dep_extractorextract_dependencies_from_macro_wrapped_code {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_dependencies_from_macro_wrapped_code"}
// Dependencies: {}
pub fn extract_dependencies_from_macro_wrapped_code (content : & str) -> HashSet < String > { let mut deps = HashSet :: new () ; if let Ok (file) = syn :: parse_file (content) { extract_deps_from_file (& file , & mut deps) ; } extract_deps_from_text (content , & mut deps) ; deps }
};
}
