// Generated macro for has_newlines_before_after_comment (function)
macro_rules! Depcrate_attrhas_newlines_before_after_comment {
() => {
// Module: crate::attr
// Provides: {"has_newlines_before_after_comment"}
// Dependencies: {}
fn has_newlines_before_after_comment (comment : & str) -> (& str , & str) { let comment_begin = comment . find ('/') ; let len = comment_begin . unwrap_or_else (| | comment . len ()) ; let mlb = count_newlines (& comment [.. len]) > 1 ; let mla = if comment_begin . is_none () { mlb } else { comment . chars () . rev () . take_while (| c | c . is_whitespace ()) . filter (| & c | c == '\n') . count () > 1 } ; (if mlb { "\n" } else { "" } , if mla { "\n" } else { "" }) }
};
}
