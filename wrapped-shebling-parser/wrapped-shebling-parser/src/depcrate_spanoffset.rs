// Generated macro for offset (function)
macro_rules! Depcrate_spanoffset {
() => {
// Module: crate::span
// Provides: {"offset"}
// Dependencies: {}
# [doc = " Returns the offset of the given span."] pub (crate) fn offset (span : ParseSpan) -> ParseResult < usize > { map (take (0usize) , | span : ParseSpan | span . offset ()) (span) }
};
}
