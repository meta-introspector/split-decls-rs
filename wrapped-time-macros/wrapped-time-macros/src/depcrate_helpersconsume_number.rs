// Generated macro for consume_number (function)
macro_rules! Depcrate_helpersconsume_number {
() => {
// Module: crate::helpers
// Provides: {"consume_number"}
// Dependencies: {}
pub (crate) fn consume_number < T : FromStr > (component_name : & 'static str , chars : & mut Peekable < token_stream :: IntoIter > ,) -> Result < (Span , T) , Error > { let (span , digits) = match chars . next () { Some (TokenTree :: Literal (literal)) => (literal . span () , literal . to_string ()) , Some (tree) => return Err (Error :: UnexpectedToken { tree }) , None => return Err (Error :: UnexpectedEndOfInput) , } ; if let Ok (value) = digits . replace ('_' , "") . parse () { Ok ((span , value)) } else { Err (Error :: InvalidComponent { name : component_name , value : digits , span_start : Some (span) , span_end : Some (span) , }) } }
};
}
