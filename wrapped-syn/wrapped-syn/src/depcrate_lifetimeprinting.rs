// Generated macro for printing (module)
macro_rules! Depcrate_lifetimeprinting {
() => {
// Module: crate::lifetime
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: ext :: PunctExt as _ ; use crate :: lifetime :: Lifetime ; use proc_macro2 :: { Punct , Spacing , TokenStream } ; use quote :: { ToTokens , TokenStreamExt as _ } ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Lifetime { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Punct :: new_spanned ('\'' , Spacing :: Joint , self . apostrophe)) ; self . ident . to_tokens (tokens) ; } } }
};
}
