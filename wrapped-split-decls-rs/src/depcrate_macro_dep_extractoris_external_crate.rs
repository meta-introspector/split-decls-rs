// Generated macro for is_external_crate (function)
macro_rules! Depcrate_macro_dep_extractoris_external_crate {
() => {
// Module: crate::macro_dep_extractor
// Provides: {"is_external_crate"}
// Dependencies: {}
fn is_external_crate (name : & str) -> bool { ! matches ! (name , "std" | "core" | "alloc" | "self" | "super" | "crate") }
};
}
