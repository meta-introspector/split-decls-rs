// Generated macro for extract_deps_from_item (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_item {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_item"}
// Dependencies: {}
fn extract_deps_from_item (item : & Item , deps : & mut HashSet < String >) { match item { Item :: Use (use_item) => { extract_deps_from_use_tree (& use_item . tree , deps) ; } Item :: Macro (macro_item) => { extract_deps_from_macro (macro_item , deps) ; } Item :: Mod (mod_item) => { if let Some ((_ , items)) = & mod_item . content { for item in items { extract_deps_from_item (item , deps) ; } } } _ => { } } }
};
}
