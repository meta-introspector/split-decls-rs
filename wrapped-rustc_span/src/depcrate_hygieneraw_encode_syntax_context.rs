// Generated macro for raw_encode_syntax_context (function)
macro_rules! Depcrate_hygieneraw_encode_syntax_context {
() => {
// Module: crate::hygiene
// Provides: {"raw_encode_syntax_context"}
// Dependencies: {}
pub fn raw_encode_syntax_context (ctxt : SyntaxContext , context : & HygieneEncodeContext , e : & mut impl Encoder ,) { if ! context . serialized_ctxts . lock () . contains (& ctxt) { context . latest_ctxts . lock () . insert (ctxt) ; } ctxt . 0 . encode (e) ; }
};
}
