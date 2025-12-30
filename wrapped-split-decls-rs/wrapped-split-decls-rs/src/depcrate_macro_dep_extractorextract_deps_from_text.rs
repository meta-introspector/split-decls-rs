// Generated macro for extract_deps_from_text (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_text {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_text"}
// Dependencies: {}
fn extract_deps_from_text (content : & str , deps : & mut HashSet < String >) { let lines : Vec < & str > = content . lines () . collect () ; let mut in_deps_macro = false ; for line in lines { let trimmed = line . trim () ; if trimmed . starts_with ("macro_rules! deps") { in_deps_macro = true ; continue ; } if in_deps_macro && trimmed == "}" { in_deps_macro = false ; continue ; } if in_deps_macro && trimmed . ends_with ("!();") { let dep_name = trimmed . trim_end_matches ("!();") . trim () ; if ! dep_name . is_empty () && is_valid_dependency (dep_name) { deps . insert (dep_name . to_string ()) ; } } } }
};
}
