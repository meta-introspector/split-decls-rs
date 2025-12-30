// Generated macro for new_at (function)
macro_rules! Depcrate_errornew_at {
() => {
// Module: crate::error
// Provides: {"new_at"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub (crate) fn new_at < T : Display > (scope : Span , cursor : Cursor , message : T) -> Error { if cursor . eof () { Error :: new (scope , format ! ("unexpected end of input, {}" , message)) } else { let span = crate :: buffer :: open_span_of_group (cursor) ; Error :: new (span , message) } }
};
}
