// Generated macro for printing (module)
macro_rules! Depcrate_fileprinting {
() => {
// Module: crate::file
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: attr :: FilterAttrs ; use crate :: file :: File ; use proc_macro2 :: TokenStream ; use quote :: { ToTokens , TokenStreamExt as _ } ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for File { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (self . attrs . inner ()) ; tokens . append_all (& self . items) ; } } }
};
}
