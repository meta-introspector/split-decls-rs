// Generated macro for brace_group (function)
macro_rules! Depcrate_parser_commandbrace_group {
() => {
// Module: crate::parser::command
// Provides: {"brace_group"}
// Dependencies: {}
pub (super) fn brace_group (span : Span) -> ParseResult < Term > { let (span , (start , space , paren)) = tuple ((terminated (position , char ('{')) , multi_trivia , followed_by (char ('(')) ,)) (span) ? ; if space . is_empty () && ! paren { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: MissingSpace) . range (& span)) ; } let (span , closing_brace) = opt (peek (preceded (char ('}') , position))) (span) ? ; if let Some (end) = closing_brace { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("empty code block" , Range :: new (start , end)) ,) ; } let (span , term) = term (span) ? ; let (span , _) = terminated (context ("missing the closing brace (or the semicolon or newline before it)" , char ('}') ,) , trivia ,) (span) ? ; Ok ((span , term)) }
};
}
