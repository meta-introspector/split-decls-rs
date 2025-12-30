// Generated macro for printing (module)
macro_rules! Depcrate_macprinting {
() => {
// Module: crate::mac
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use quote :: { Tokens , ToTokens } ; impl ToTokens for Mac { fn to_tokens (& self , tokens : & mut Tokens) { self . path . to_tokens (tokens) ; self . bang_token . to_tokens (tokens) ; self . ident . to_tokens (tokens) ; tokens . append_all (& self . tokens) ; } } impl ToTokens for TokenTree { fn to_tokens (& self , tokens : & mut Tokens) { self . 0 . to_tokens (tokens) ; } } }
};
}
