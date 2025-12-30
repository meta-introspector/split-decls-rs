// Generated macro for case_cmd (function)
macro_rules! Depcrate_parser_commandcase_cmd {
() => {
// Module: crate::parser::command
// Provides: {"case_cmd"}
// Dependencies: {}
fn case_cmd (span : Span) -> ParseResult < CaseCmd > { terminated (map (pair (delimited (pair (token (Keyword :: Case) , trivia) , word , tuple ((multi_trivia , cut (context ("expected 'in'!" , token (Keyword :: In))) , trivia , linebreak ,)) ,) , many0 (case_clause) ,) , | (word , clauses) | CaseCmd :: new (word , clauses) ,) , cut (context ("expected a closing 'esac'!" , token (Keyword :: Esac))) ,) (span) }
};
}
