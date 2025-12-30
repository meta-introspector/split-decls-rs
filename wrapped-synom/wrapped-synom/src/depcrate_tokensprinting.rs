// Generated macro for printing (module)
macro_rules! Depcrate_tokensprinting {
() => {
// Module: crate::tokens
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use proc_macro2 :: { TokenTree , TokenNode , Spacing , Term } ; use quote :: Tokens ; use span :: Span ; pub fn op (s : & str , spans : & [Span] , tokens : & mut Tokens) { assert_eq ! (s . len () , spans . len ()) ; let mut chars = s . chars () ; let mut spans = spans . iter () ; let ch = chars . next_back () . unwrap () ; let span = spans . next_back () . unwrap () ; for (ch , span) in chars . zip (spans) { tokens . append (TokenTree { span : span . 0 , kind : TokenNode :: Op (ch , Spacing :: Joint) , }) ; } tokens . append (TokenTree { span : span . 0 , kind : TokenNode :: Op (ch , Spacing :: Alone) , }) ; } pub fn sym (s : & str , span : & Span , tokens : & mut Tokens) { tokens . append (TokenTree { span : span . 0 , kind : TokenNode :: Term (Term :: intern (s)) , }) ; } pub fn delim < F > (s : & str , span : & Span , tokens : & mut Tokens , f : F) where F : FnOnce (& mut Tokens) { tokens . append_delimited (s , span . 0 , f) } }
};
}
