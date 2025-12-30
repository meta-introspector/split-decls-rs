// Generated macro for span_of_unexpected_ignoring_nones (function)
macro_rules! Depcrate_parsespan_of_unexpected_ignoring_nones {
() => {
// Module: crate::parse
// Provides: {"span_of_unexpected_ignoring_nones"}
// Dependencies: {}
fn span_of_unexpected_ignoring_nones (mut cursor : Cursor) -> Option < (Span , Delimiter) > { if cursor . eof () { return None ; } while let Some ((inner , _span , rest)) = cursor . group (Delimiter :: None) { if let Some (unexpected) = span_of_unexpected_ignoring_nones (inner) { return Some (unexpected) ; } cursor = rest ; } if cursor . eof () { None } else { Some ((cursor . span () , cursor . scope_delimiter ())) } }
};
}
