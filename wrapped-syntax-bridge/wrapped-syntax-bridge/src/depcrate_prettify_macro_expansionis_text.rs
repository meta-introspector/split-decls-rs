// Generated macro for is_text (function)
macro_rules! Depcrate_prettify_macro_expansionis_text {
() => {
// Module: crate::prettify_macro_expansion
// Provides: {"is_text"}
// Dependencies: {}
fn is_text (k : SyntaxKind) -> bool { k . is_any_identifier () || k . is_literal () || k == UNDERSCORE }
};
}
