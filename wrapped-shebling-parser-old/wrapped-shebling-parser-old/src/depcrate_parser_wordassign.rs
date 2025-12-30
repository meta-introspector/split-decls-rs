// Generated macro for assign (function)
macro_rules! Depcrate_parser_wordassign {
() => {
// Module: crate::parser::word
// Provides: {"assign"}
// Dependencies: {}
pub (super) fn assign (span : Span) -> ParseResult < Assign > { let (span , (ident , subscripts)) = preceded (context ("don't use $ on the left side of assignments!" , not (char ('$')) ,) , pair (context ("invalid identifier!" , identifier) , many0 (subscript)) ,) (span) ? ; let (span , op) = context ("expected an assignment operator!" , alt ((token (BinOp :: AddEq) , token (BinOp :: Eq))) ,) (span) ? ; let (span , trailing) = opt (peek (ranged (token (BinOp :: Eq)))) (span) ? ; let (span , ((right_space , space_range) , end_of_cmd)) = pair (ranged (map (trivia , | trivia | ! trivia . is_empty ())) , followed_by (alt ((is_a ("\r\n;&|)") , eof))) ,) (span) ? ; let (span , value) = if right_space || end_of_cmd { if ident != "IFS" && right_space && ! end_of_cmd { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusValue) . label ("space causes the value to be empty" , space_range) . help ("If you do want a value, remove the space. Else use '' for an empty value." ,) ,) ; } (span , Value :: Empty) } else { if let Some ((_ , range)) = trailing { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusValue) . label ("this = is part of the value" , range) . help ("Use a single = for assignments, or [ .. ] / [[ .. ]] for comparisons.") ,) ; } terminated (alt ((into (array) , into (word))) , trivia) (span) ? } ; Ok ((span , Assign :: new (SubscriptedVar :: new (ident , subscripts) , value , op) ,)) }
};
}
