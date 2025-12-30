// Generated macro for printing (module)
macro_rules! Depcrate_lifetimeprinting {
() => {
// Module: crate::lifetime
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use quote :: { Tokens , ToTokens } ; use proc_macro2 :: { TokenTree , TokenNode } ; impl ToTokens for Lifetime { fn to_tokens (& self , tokens : & mut Tokens) { tokens . append (TokenTree { span : self . span . 0 , kind : TokenNode :: Term (self . sym) , }) } } }
};
}
