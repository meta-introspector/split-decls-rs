// Generated macro for to_valid_ident (function)
macro_rules! Depcrate_identifierto_valid_ident {
() => {
// Module: crate::identifier
// Provides: {"to_valid_ident"}
// Dependencies: {}
# [doc = " Converts a string to a valid JavaScript identifier by replacing invalid"] # [doc = " characters with underscores."] pub fn to_valid_ident (name : & str) -> String { maybe_valid_chars (name) . map (| opt | opt . unwrap_or ('_')) . collect () }
};
}
