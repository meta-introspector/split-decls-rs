// Generated macro for with_span_interner (function)
macro_rules! Depcrate_span_encodingwith_span_interner {
() => {
// Module: crate::span_encoding
// Provides: {"with_span_interner"}
// Dependencies: {}
# [inline] fn with_span_interner < T , F : FnOnce (& mut SpanInterner) -> T > (f : F) -> T { crate :: with_session_globals (| session_globals | f (& mut session_globals . span_interner . lock ())) }
};
}
