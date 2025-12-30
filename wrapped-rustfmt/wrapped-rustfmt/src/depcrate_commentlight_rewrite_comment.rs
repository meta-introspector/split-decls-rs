// Generated macro for light_rewrite_comment (function)
macro_rules! Depcrate_commentlight_rewrite_comment {
() => {
// Module: crate::comment
// Provides: {"light_rewrite_comment"}
// Dependencies: {}
# [doc = " Trims whitespace and aligns to indent, but otherwise does not change comments."] fn light_rewrite_comment (orig : & str , offset : Indent , config : & Config , is_doc_comment : bool ,) -> String { orig . lines () . map (| l | { let first_non_whitespace = l . find (| c | ! char :: is_whitespace (c)) ; let left_trimmed = if let Some (fnw) = first_non_whitespace { if l . as_bytes () [fnw] == b'*' && fnw > 0 { & l [fnw - 1 ..] } else { & l [fnw ..] } } else { "" } ; trim_end_unless_two_whitespaces (left_trimmed , is_doc_comment) }) . join (& format ! ("\n{}" , offset . to_string (config))) }
};
}
