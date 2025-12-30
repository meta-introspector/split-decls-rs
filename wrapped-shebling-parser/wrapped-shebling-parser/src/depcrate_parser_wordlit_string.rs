// Generated macro for lit_string (function)
macro_rules! Depcrate_parser_wordlit_string {
() => {
// Module: crate::parser::word
// Provides: {"lit_string"}
// Dependencies: {}
fn lit_string (end_pattern : & 'static str) -> impl Fn (ParseSpan) -> ParseResult < String > { move | span | { map (many1 (alt ((preceded (backslash , into (alt ((line_space , one_of (& * format ! ("|&;<>()'\n\r[]{{}}.,~#{}{}" , expansion :: EXTGLOB_PREFIX , quoted :: DOUBLE_ESCAPABLE)) ,))) ,) , recognize_string (is_not (& * format ! ("[{{}}|&;<>() '\t\n\r\u{A0}{}{}{}" , expansion :: EXTGLOB_PREFIX , quoted :: DOUBLE_ESCAPABLE , end_pattern))) ,))) , | lits | lits . concat () ,) (span) } }
};
}
