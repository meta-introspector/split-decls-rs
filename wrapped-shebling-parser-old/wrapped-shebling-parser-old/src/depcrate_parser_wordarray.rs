// Generated macro for array (function)
macro_rules! Depcrate_parser_wordarray {
() => {
// Module: crate::parser::word
// Provides: {"array"}
// Dependencies: {}
fn array (span : Span) -> ParseResult < Array > { let (span , (first_paren , second_paren)) = terminated (pair (terminated (position , char ('(')) , opt (peek (preceded (char ('(') , position))) ,) , multi_trivia ,) (span) ? ; if let Some (second_paren) = second_paren { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusValue) . range (Range :: new (first_paren , second_paren)) . help ("Doing math? Then you're missing the dollar in $((..)), or use '( (' for nested arrays.") ,) ; } let (span , elems) = many0 (delimited (not (char (')')) , alt ((map (separated_pair (many1 (subscript) , char ('=') , alt ((into (array) , into (word) , | span | Ok ((span , Value :: Empty)))) ,) , | (key , value) | KeyValue :: new (key , value) . into () ,) , into (array) , into (word) ,)) , multi_trivia ,)) (span) ? ; let (span , _) = context ("expected a closing ')' for this array!" , char (')')) (span) ? ; Ok ((span , Array :: new (elems))) }
};
}
