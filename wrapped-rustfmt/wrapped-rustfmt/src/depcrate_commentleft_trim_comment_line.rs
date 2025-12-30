// Generated macro for left_trim_comment_line (function)
macro_rules! Depcrate_commentleft_trim_comment_line {
() => {
// Module: crate::comment
// Provides: {"left_trim_comment_line"}
// Dependencies: {}
# [doc = " Trims comment characters and possibly a single space from the left of a string."] # [doc = " Does not trim all whitespace. If a single space is trimmed from the left of the string,"] # [doc = " this function returns true."] fn left_trim_comment_line < 'a > (line : & 'a str , style : & CommentStyle < '_ >) -> (& 'a str , bool) { if line . starts_with ("//! ") || line . starts_with ("/// ") || line . starts_with ("/*! ") || line . starts_with ("/** ") { (& line [4 ..] , true) } else if let CommentStyle :: Custom (opener) = * style { if let Some (stripped) = line . strip_prefix (opener) { (stripped , true) } else { (& line [opener . trim_end () . len () ..] , false) } } else if line . starts_with ("/* ") || line . starts_with ("// ") || line . starts_with ("//!") || line . starts_with ("///") || line . starts_with ("** ") || line . starts_with ("/*!") || (line . starts_with ("/**") && ! line . starts_with ("/**/")) { (& line [3 ..] , line . chars () . nth (2) . unwrap () == ' ') } else if line . starts_with ("/*") || line . starts_with ("* ") || line . starts_with ("//") || line . starts_with ("**") { (& line [2 ..] , line . chars () . nth (1) . unwrap () == ' ') } else if let Some (stripped) = line . strip_prefix ('*') { (stripped , false) } else { (line , line . starts_with (' ')) } }
};
}
