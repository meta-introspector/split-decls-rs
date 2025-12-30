// Generated macro for lit_word_sgmt (function)
macro_rules! Depcrate_parser_wordlit_word_sgmt {
() => {
// Module: crate::parser::word
// Provides: {"lit_word_sgmt"}
// Dependencies: {}
pub (super) fn lit_word_sgmt < 'a > (end_pattern : & 'static str ,) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < WordSgmt > { map (spanned (lit_string (end_pattern)) , WordSgmt :: Lit) }
};
}
