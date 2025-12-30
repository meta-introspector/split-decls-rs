// Generated macro for printing (module)
macro_rules! Depcrate_punctuatedprinting {
() => {
// Module: crate::punctuated
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: punctuated :: { Pair , Punctuated } ; use proc_macro2 :: TokenStream ; use quote :: { ToTokens , TokenStreamExt as _ } ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl < T , P > ToTokens for Punctuated < T , P > where T : ToTokens , P : ToTokens , { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (self . pairs ()) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl < T , P > ToTokens for Pair < T , P > where T : ToTokens , P : ToTokens , { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Pair :: Punctuated (a , b) => { a . to_tokens (tokens) ; b . to_tokens (tokens) ; } Pair :: End (a) => a . to_tokens (tokens) , } } } }
};
}
