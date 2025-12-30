// Generated macro for printing (module)
macro_rules! Depcrate_macprinting {
() => {
// Module: crate::mac
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: mac :: { Macro , MacroDelimiter } ; use crate :: path ; use crate :: path :: printing :: PathStyle ; use crate :: token ; use proc_macro2 :: { Delimiter , TokenStream } ; use quote :: ToTokens ; impl MacroDelimiter { pub (crate) fn surround (& self , tokens : & mut TokenStream , inner : TokenStream) { let (delim , span) = match self { MacroDelimiter :: Paren (paren) => (Delimiter :: Parenthesis , paren . span) , MacroDelimiter :: Brace (brace) => (Delimiter :: Brace , brace . span) , MacroDelimiter :: Bracket (bracket) => (Delimiter :: Bracket , bracket . span) , } ; token :: printing :: delim (delim , span . join () , tokens , inner) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Macro { fn to_tokens (& self , tokens : & mut TokenStream) { path :: printing :: print_path (tokens , & self . path , PathStyle :: Mod) ; self . bang_token . to_tokens (tokens) ; self . delimiter . surround (tokens , self . tokens . clone ()) ; } } }
};
}
