// Generated macro for word_sgmt_before_pattern (function)
macro_rules! Depcrate_parser_wordword_sgmt_before_pattern {
() => {
// Module: crate::parser::word
// Provides: {"word_sgmt_before_pattern"}
// Dependencies: {}
fn word_sgmt_before_pattern < 'a > (pattern : & 'static str ,) -> impl FnMut (Span < 'a >) -> ParseResult < WordSgmt > { fn lit_curly (span : Span) -> ParseResult < String > { let (span , (curly , range)) = ranged (one_of ("{}")) (span) ? ; span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("literal curly" , range) . help ("If intended, quote it. Else add a semicolon or new line before it.") ,) ; Ok ((span , curly . into ())) } move | span | { let (span , _) = not (one_of (pattern)) (span) ? ; let (span , _) = context ("forgot to escape this parenthesis?" , not (char ('('))) (span) ? ; alt ((map (single_quoted , WordSgmt :: SingleQuoted) , into (double_quoted) , map (alt ((extglob , recognize_string (one_of ("*?")) , bracketed_glob)) , WordSgmt :: Glob ,) , map (one_of ("@!+[") , | c | WordSgmt :: Lit (c . into ())) , dollar_sgmt , map (brace_expansion , WordSgmt :: BraceExpansion) , map (backquoted , WordSgmt :: BackQuoted) , map (proc_sub , WordSgmt :: ProcSub) , lit_word_sgmt (pattern) , map (alt ((recognize_string (tag ("{}")) , lit_curly)) , WordSgmt :: Lit) ,)) (span) } }
};
}
