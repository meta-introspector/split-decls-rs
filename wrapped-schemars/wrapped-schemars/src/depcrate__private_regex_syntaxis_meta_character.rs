// Generated macro for is_meta_character (function)
macro_rules! Depcrate__private_regex_syntaxis_meta_character {
() => {
// Module: crate::_private::regex_syntax
// Provides: {"is_meta_character"}
// Dependencies: {}
fn is_meta_character (c : char) -> bool { match c { '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '#' | '&' | '-' | '~' => true , _ => false , } }
};
}
