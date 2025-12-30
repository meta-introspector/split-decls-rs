// Generated macro for extract_deps_from_macro (function)
macro_rules! Depcrate_macro_dep_extractorextract_deps_from_macro {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"extract_deps_from_macro"}
// Dependencies: {}
fn extract_deps_from_macro (macro_item : & syn :: ItemMacro , deps : & mut HashSet < String >) { if let Some (ident) = & macro_item . ident { if ident == "deps" { extract_deps_from_macro_body (& macro_item . mac . tokens , deps) ; } } }
};
}
