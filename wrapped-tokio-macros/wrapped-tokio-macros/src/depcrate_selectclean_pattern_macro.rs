// Generated macro for clean_pattern_macro (function)
macro_rules! Depcrate_selectclean_pattern_macro {
() => {
// Module: crate::select
// Provides: {"clean_pattern_macro"}
// Dependencies: {}
pub (crate) fn clean_pattern_macro (input : TokenStream) -> TokenStream { let mut input : syn :: Pat = match syn :: Pat :: parse_single . parse (input . clone ()) { Ok (it) => it , Err (_) => return input , } ; clean_pattern (& mut input) ; quote :: ToTokens :: into_token_stream (input) . into () }
};
}
