// Generated macro for has_extra_newline (function)
macro_rules! Depcrate_listshas_extra_newline {
() => {
// Module: crate::lists
// Provides: {"has_extra_newline"}
// Dependencies: {}
pub (crate) fn has_extra_newline (post_snippet : & str , comment_end : usize) -> bool { if post_snippet . is_empty () || comment_end == 0 { return false ; } let len_last = post_snippet [.. comment_end] . chars () . last () . unwrap () . len_utf8 () ; let test_snippet = & post_snippet [comment_end - len_last ..] ; let first_newline = test_snippet . find ('\n') . unwrap_or_else (| | test_snippet . len ()) ; let test_snippet = & test_snippet [first_newline ..] ; let first = test_snippet . find (| c : char | ! c . is_whitespace ()) . unwrap_or_else (| | test_snippet . len ()) ; let test_snippet = & test_snippet [.. first] ; count_newlines (test_snippet) > 1 }
};
}
