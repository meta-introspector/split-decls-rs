// Generated macro for tokens_to_parse_buffer (function)
macro_rules! Depcrate_parsetokens_to_parse_buffer {
() => {
// Module: crate::parse
// Provides: {"tokens_to_parse_buffer"}
// Dependencies: {}
fn tokens_to_parse_buffer (tokens : & TokenBuffer) -> ParseBuffer { let scope = Span :: call_site () ; let cursor = tokens . begin () ; let unexpected = Rc :: new (Cell :: new (Unexpected :: None)) ; new_parse_buffer (scope , cursor , unexpected) }
};
}
