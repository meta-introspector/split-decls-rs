// Generated macro for capture_format_string_idents (function)
macro_rules! Depcrate_macros_strings_displaycapture_format_string_idents {
() => {
// Module: crate::macros::strings::display
// Provides: {"capture_format_string_idents"}
// Dependencies: {}
fn capture_format_string_idents (string_literal : & LitStr) -> syn :: Result < Vec < Ident > > { capture_format_strings (string_literal) ? . into_iter () . map (| ident | { syn :: parse_str :: < Ident > (ident . as_str ()) . map_err (| _ | { syn :: Error :: new_spanned (string_literal , "Invalid identifier inside format string bracket" ,) }) }) . collect () }
};
}
