// Generated macro for comment (function)
macro_rules! Depcrate_generatorcomment {
() => {
// Module: crate::generator
// Provides: {"comment"}
// Dependencies: {}
fn comment (mut comment : String , features : & Option < String >) -> TokenStream { if let Some (s) = features { comment . push_str (s) ; } let lines = comment . lines () . map (| doc | quote ! (# [doc = # doc])) ; quote ! { # (# lines) * } }
};
}
