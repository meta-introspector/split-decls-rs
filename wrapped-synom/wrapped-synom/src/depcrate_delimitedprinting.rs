// Generated macro for printing (module)
macro_rules! Depcrate_delimitedprinting {
() => {
// Module: crate::delimited
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use quote :: { Tokens , ToTokens } ; impl < T , D > ToTokens for Delimited < T , D > where T : ToTokens , D : ToTokens , { fn to_tokens (& self , tokens : & mut Tokens) { tokens . append_all (self . iter ()) } } impl < T , D > ToTokens for Element < T , D > where T : ToTokens , D : ToTokens , { fn to_tokens (& self , tokens : & mut Tokens) { match * self { Element :: Delimited (ref a , ref b) => { a . to_tokens (tokens) ; b . to_tokens (tokens) ; } Element :: End (ref a) => a . to_tokens (tokens) , } } } }
};
}
