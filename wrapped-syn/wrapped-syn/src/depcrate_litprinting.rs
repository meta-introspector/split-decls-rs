// Generated macro for printing (module)
macro_rules! Depcrate_litprinting {
() => {
// Module: crate::lit
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use quote :: { Tokens , ToTokens } ; impl ToTokens for Lit { fn to_tokens (& self , tokens : & mut Tokens) { self . clone () . into_token_tree () . to_tokens (tokens) } } }
};
}
