// Generated macro for extract_deps_from_file (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_file {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_file"}
// Dependencies: {}
fn extract_deps_from_file (file : & File , deps : & mut HashSet < String >) { for item in & file . items { extract_deps_from_item (item , deps) ; } }
};
}
