// Generated macro for printing (module)
macro_rules! Depcrate_lifetimeprinting {
() => {
// Module: crate::lifetime
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: lifetime :: Lifetime ; use proc_macro2 :: { Punct , Spacing , TokenStream } ; use quote :: { ToTokens , TokenStreamExt } ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Lifetime { fn to_tokens (& self , tokens : & mut TokenStream) { let mut apostrophe = Punct :: new ('\'' , Spacing :: Joint) ; apostrophe . set_span (self . apostrophe) ; tokens . append (apostrophe) ; self . ident . to_tokens (tokens) ; } } }
};
}
