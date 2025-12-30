// Generated macro for extract_deps_from_use_tree (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_use_tree {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_use_tree"}
// Dependencies: {}
fn extract_deps_from_use_tree (tree : & UseTree , deps : & mut HashSet < String >) { match tree { UseTree :: Path (UsePath { ident , tree , .. }) => { let crate_name = ident . to_string () ; if is_external_crate (& crate_name) { deps . insert (crate_name) ; } extract_deps_from_use_tree (tree , deps) ; } UseTree :: Group (UseGroup { items , .. }) => { for item in items { extract_deps_from_use_tree (item , deps) ; } } UseTree :: Glob (UseGlob { .. }) => { } UseTree :: Name (name) => { let crate_name = name . ident . to_string () ; if is_external_crate (& crate_name) { deps . insert (crate_name) ; } } UseTree :: Rename (UseRename { ident , .. }) => { let crate_name = ident . to_string () ; if is_external_crate (& crate_name) { deps . insert (crate_name) ; } } } }
};
}
