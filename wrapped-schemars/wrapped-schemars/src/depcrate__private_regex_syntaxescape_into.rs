// Generated macro for escape_into (function)
macro_rules! Depcrate__private_regex_syntaxescape_into {
() => {
// Module: crate::_private::regex_syntax
// Provides: {"escape_into"}
// Dependencies: {}
fn escape_into (text : & str , buf : & mut String) { buf . reserve (text . len ()) ; for c in text . chars () { if is_meta_character (c) { buf . push ('\\') ; } buf . push (c) ; } }
};
}
