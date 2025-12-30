// Generated macro for extract_post_comment (function)
macro_rules! Depcrate_listsextract_post_comment {
() => {
// Module: crate::lists
// Provides: {"extract_post_comment"}
// Dependencies: {}
pub (crate) fn extract_post_comment (post_snippet : & str , comment_end : usize , separator : & str , is_last : bool ,) -> Option < String > { let white_space : & [_] = & [' ' , '\t'] ; let post_snippet = post_snippet [.. comment_end] . trim () ; let last_inline_comment_ends_with_separator = if is_last { if let Some (line) = post_snippet . lines () . last () { line . ends_with (separator) && line . trim () . starts_with ("//") } else { false } } else { false } ; let post_snippet_trimmed = if post_snippet . starts_with (| c | c == ',' || c == ':') { post_snippet [1 ..] . trim_matches (white_space) } else if let Some (stripped) = post_snippet . strip_prefix (separator) { stripped . trim_matches (white_space) } else if last_inline_comment_ends_with_separator { post_snippet . trim_matches (white_space) } else if post_snippet . ends_with (separator) && (! post_snippet . trim () . starts_with ("//") || post_snippet . trim () . contains ('\n')) { post_snippet [.. (post_snippet . len () - 1)] . trim_matches (white_space) } else { post_snippet } ; let removed_newline_snippet = post_snippet_trimmed . trim () ; if ! post_snippet_trimmed . is_empty () && (removed_newline_snippet . starts_with ("//") || removed_newline_snippet . starts_with ("/*")) { Some (post_snippet_trimmed . to_owned ()) } else { None } }
};
}
