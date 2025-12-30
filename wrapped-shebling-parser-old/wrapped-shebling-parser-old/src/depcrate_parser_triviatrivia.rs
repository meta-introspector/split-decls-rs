// Generated macro for trivia (function)
macro_rules! Depcrate_parser_triviatrivia {
() => {
// Module: crate::parser::trivia
// Provides: {"trivia"}
// Dependencies: {}
pub (super) fn trivia (span : Span) -> ParseResult < String > { fn continued (span : Span) -> ParseResult < Vec < char > > { let (span , (mut continued , comment)) = preceded (line_continuation , pair (many0 (line_space) , opt (comment)) ,) (span) ? ; if let Some (comment) = comment { if comment . ends_with ('\\') { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: BadEscape) . label ("this backslash is part of a comment" , & span) ,) ; } continued . append (& mut comment . chars () . collect ()) ; } Ok ((span , continued)) } map (pair (map (many0 (alt ((many1 (line_space) , continued))) , | trivia | { trivia . into_iter () . flatten () . collect :: < String > () }) , opt (comment) ,) , | (mut trivia , comment) | { trivia . extend (comment) ; trivia } ,) (span) }
};
}
