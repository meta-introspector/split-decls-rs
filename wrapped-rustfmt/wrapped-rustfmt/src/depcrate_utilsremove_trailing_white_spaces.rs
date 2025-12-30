// Generated macro for remove_trailing_white_spaces (function)
macro_rules! Depcrate_utilsremove_trailing_white_spaces {
() => {
// Module: crate::utils
// Provides: {"remove_trailing_white_spaces"}
// Dependencies: {}
# [doc = " Removes trailing spaces from the specified snippet. We do not remove spaces"] # [doc = " inside strings or comments."] pub (crate) fn remove_trailing_white_spaces (text : & str) -> String { let mut buffer = String :: with_capacity (text . len ()) ; let mut space_buffer = String :: with_capacity (128) ; for (char_kind , c) in CharClasses :: new (text . chars ()) { match c { '\n' => { if char_kind == FullCodeCharKind :: InString { buffer . push_str (& space_buffer) ; } space_buffer . clear () ; buffer . push ('\n') ; } _ if c . is_whitespace () => { space_buffer . push (c) ; } _ => { if ! space_buffer . is_empty () { buffer . push_str (& space_buffer) ; space_buffer . clear () ; } buffer . push (c) ; } } } buffer }
};
}
