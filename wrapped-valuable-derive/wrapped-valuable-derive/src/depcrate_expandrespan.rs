// Generated macro for respan (function)
macro_rules! Depcrate_expandrespan {
() => {
// Module: crate::expand
// Provides: {"respan"}
// Dependencies: {}
fn respan (tokens : TokenStream , span : & impl ToTokens) -> TokenStream { let mut iter = span . to_token_stream () . into_iter () ; let start_span = iter . next () . map_or_else (Span :: call_site , | t | t . span ()) ; let end_span = iter . last () . map_or (start_span , | t | t . span ()) ; let mut tokens = tokens . into_iter () . collect :: < Vec < _ > > () ; if let Some (tt) = tokens . first_mut () { tt . set_span (start_span) ; } for tt in tokens . iter_mut () . skip (1) { tt . set_span (end_span) ; } tokens . into_iter () . collect () }
};
}
