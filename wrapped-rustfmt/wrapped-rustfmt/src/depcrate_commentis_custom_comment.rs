// Generated macro for is_custom_comment (function)
macro_rules! Depcrate_commentis_custom_comment {
() => {
// Module: crate::comment
// Provides: {"is_custom_comment"}
// Dependencies: {}
fn is_custom_comment (comment : & str) -> bool { if ! comment . starts_with ("//") { false } else if let Some (c) = comment . chars () . nth (2) { ! c . is_alphanumeric () && ! c . is_whitespace () } else { false } }
};
}
