// Generated macro for return_macro_parse_failure_fallback (function)
macro_rules! Depcrate_macrosreturn_macro_parse_failure_fallback {
() => {
// Module: crate::macros
// Provides: {"return_macro_parse_failure_fallback"}
// Dependencies: {}
fn return_macro_parse_failure_fallback (context : & RewriteContext < '_ > , indent : Indent , position : MacroPosition , span : Span ,) -> RewriteResult { context . macro_rewrite_failure . replace (true) ; let is_like_block_indent_style = context . snippet (span) . lines () . last () . map (| closing_line | { closing_line . trim () . chars () . all (| ch | matches ! (ch , '}' | ')' | ']')) }) . unwrap_or (false) ; if is_like_block_indent_style { return trim_left_preserve_layout (context . snippet (span) , indent , context . config) . macro_error (MacroErrorKind :: Unknown , span) ; } context . skipped_range . borrow_mut () . push ((context . psess . line_of_byte_pos (span . lo ()) , context . psess . line_of_byte_pos (span . hi ()) ,)) ; let mut snippet = context . snippet (span) . to_owned () ; if position == MacroPosition :: Item { snippet . push (';') ; } Ok (snippet) }
};
}
