// Generated macro for source_to_span (function)
macro_rules! Depcratesource_to_span {
() => {
// Module: crate
// Provides: {"source_to_span"}
// Dependencies: {}
fn source_to_span (source_code : & str) -> Span { let context = ParseContext { diags : RefCell :: new (vec ! []) , } ; nom_locate :: LocatedSpan :: new_extra (source_code , context) }
};
}
