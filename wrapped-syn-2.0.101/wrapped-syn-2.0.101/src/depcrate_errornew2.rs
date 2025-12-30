// Generated macro for new2 (function)
macro_rules! Depcrate_errornew2 {
() => {
// Module: crate::error
// Provides: {"new2"}
// Dependencies: {}
# [cfg (all (feature = "parsing" , any (feature = "full" , feature = "derive")))] pub (crate) fn new2 < T : Display > (start : Span , end : Span , message : T) -> Error { return new2 (start , end , message . to_string ()) ; fn new2 (start : Span , end : Span , message : String) -> Error { Error { messages : vec ! [ErrorMessage { span : ThreadBound :: new (SpanRange { start , end }) , message , }] , } } }
};
}
