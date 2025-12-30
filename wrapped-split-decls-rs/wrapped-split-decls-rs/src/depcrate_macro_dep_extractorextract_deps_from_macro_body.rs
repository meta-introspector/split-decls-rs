// Generated macro for extract_deps_from_macro_body (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_macro_body {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_macro_body"}
// Dependencies: {}
fn extract_deps_from_macro_body (tokens : & TokenStream , deps : & mut HashSet < String >) { let token_string = tokens . to_string () ; for line in token_string . lines () { let line = line . trim () ; if line . ends_with ("!();") { let dep_name = line . trim_end_matches ("!();") . trim () ; if ! dep_name . is_empty () && is_valid_dependency (dep_name) { deps . insert (dep_name . to_string ()) ; } } } }
};
}
