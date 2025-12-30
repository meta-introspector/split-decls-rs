// Generated macro for word_sgmt_before_pattern (function)
macro_rules! Depcrate_parser_wordword_sgmt_before_pattern {
() => {
// Module: crate::parser::word
// Provides: {"word_sgmt_before_pattern"}
// Dependencies: {}
fn word_sgmt_before_pattern < 'a > (pattern : & 'static str ,) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < WordSgmt > { alt ((map (spanned (single_quoted) , WordSgmt :: SingleQuoted) , into (double_quoted) , map (spanned (alt ((extglob , recognize_string (one_of ("*?")) , bracketed_glob ,))) , WordSgmt :: Glob ,) , map (spanned (recognize_string (one_of ("@!+["))) , WordSgmt :: Lit) , dollar_sgmt , map (brace_expansion , WordSgmt :: BraceExpansion) , lit_word_sgmt (pattern) , map (spanned (recognize_string (alt ((tag ("{}") , tag ("{") , tag ("}"))))) , WordSgmt :: Lit ,) ,)) }
};
}
