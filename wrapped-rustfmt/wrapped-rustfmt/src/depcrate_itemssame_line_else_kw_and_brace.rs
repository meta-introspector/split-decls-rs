// Generated macro for same_line_else_kw_and_brace (function)
macro_rules! Depcrate_itemssame_line_else_kw_and_brace {
() => {
// Module: crate::items
// Provides: {"same_line_else_kw_and_brace"}
// Dependencies: {}
# [doc = " When the initializer expression is multi-lined, then the else keyword and opening brace of the"] # [doc = " block ( i.e. \"else {\") should be put on the same line as the end of the initializer expression"] # [doc = " if all the following are true:"] # [doc = ""] # [doc = " 1. The initializer expression ends with one or more closing parentheses, square brackets,"] # [doc = "    or braces"] # [doc = " 2. There is nothing else on that line"] # [doc = " 3. That line is not indented beyond the indent on the first line of the let keyword"] fn same_line_else_kw_and_brace (init_str : & str , context : & RewriteContext < '_ > , else_kw_span : Span , init_shape : Shape ,) -> bool { if ! init_str . contains ('\n') { return init_shape . width . saturating_sub (init_str . len ()) >= 7 ; } if ! init_str . ends_with ([')' , ']' , '}']) { return false ; } let else_kw_snippet = context . snippet (else_kw_span) . trim () ; if else_kw_snippet != "else" { return false ; } let indent = init_shape . indent . to_string (context . config) ; init_str . lines () . last () . expect ("initializer expression is multi-lined") . strip_prefix (indent . as_ref ()) . map_or (false , | l | ! l . starts_with (char :: is_whitespace)) }
};
}
