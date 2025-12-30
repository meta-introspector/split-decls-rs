// Generated macro for printing (module)
macro_rules! Depcrate_tokenprinting {
() => {
// Module: crate::token
// Provides: {"printing"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "printing")] pub (crate) mod printing { use proc_macro2 :: { Delimiter , Group , Ident , Punct , Spacing , Span , TokenStream } ; use quote :: TokenStreamExt ; # [doc (hidden)] pub fn punct (s : & str , spans : & [Span] , tokens : & mut TokenStream) { assert_eq ! (s . len () , spans . len ()) ; let mut chars = s . chars () ; let mut spans = spans . iter () ; let ch = chars . next_back () . unwrap () ; let span = spans . next_back () . unwrap () ; for (ch , span) in chars . zip (spans) { let mut op = Punct :: new (ch , Spacing :: Joint) ; op . set_span (* span) ; tokens . append (op) ; } let mut op = Punct :: new (ch , Spacing :: Alone) ; op . set_span (* span) ; tokens . append (op) ; } pub (crate) fn keyword (s : & str , span : Span , tokens : & mut TokenStream) { tokens . append (Ident :: new (s , span)) ; } pub (crate) fn delim (delim : Delimiter , span : Span , tokens : & mut TokenStream , inner : TokenStream ,) { let mut g = Group :: new (delim , inner) ; g . set_span (span) ; tokens . append (g) ; } }
};
}
