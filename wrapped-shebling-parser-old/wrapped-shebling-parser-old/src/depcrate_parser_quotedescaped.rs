// Generated macro for escaped (function)
macro_rules! Depcrate_parser_quotedescaped {
() => {
// Module: crate::parser::quoted
// Provides: {"escaped"}
// Dependencies: {}
pub (super) fn escaped < 'a > (can_escape : & 'static str) -> impl FnMut (Span < 'a >) -> ParseResult < String > { alt ((map (line_continuation , | _ | String :: new ()) , map (pair (backslash , anychar) , move | (bs , c) | { if can_escape . contains (c) { c . into () } else { format ! ("{}{}" , bs , c) } }) ,)) }
};
}
