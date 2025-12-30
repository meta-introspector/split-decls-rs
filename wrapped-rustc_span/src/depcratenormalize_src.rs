// Generated macro for normalize_src (function)
macro_rules! Depcratenormalize_src {
() => {
// Module: crate
// Provides: {"normalize_src"}
// Dependencies: {}
# [doc = " Normalizes the source code and records the normalizations."] fn normalize_src (src : & mut String) -> Vec < NormalizedPos > { let mut normalized_pos = vec ! [] ; remove_bom (src , & mut normalized_pos) ; normalize_newlines (src , & mut normalized_pos) ; normalized_pos }
};
}
