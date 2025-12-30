// Generated macro for respan_token (function)
macro_rules! Depcrate_internals_respanrespan_token {
() => {
// Module: crate::internals::respan
// Provides: {"respan_token"}
// Dependencies: {}
fn respan_token (mut token : TokenTree , span : Span) -> TokenTree { if let TokenTree :: Group (g) = & mut token { * g = Group :: new (g . delimiter () , respan (g . stream () , span)) ; } token . set_span (span) ; token }
};
}
