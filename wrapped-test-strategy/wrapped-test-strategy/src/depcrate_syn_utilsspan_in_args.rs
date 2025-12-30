// Generated macro for span_in_args (function)
macro_rules! Depcrate_syn_utilsspan_in_args {
() => {
// Module: crate::syn_utils
// Provides: {"span_in_args"}
// Dependencies: {}
pub fn span_in_args (meta : & Meta) -> Span { match meta { Meta :: Path (_) => meta . span () , Meta :: List (m) => m . delimiter . span () . span () , Meta :: NameValue (m) => m . value . span () , } }
};
}
