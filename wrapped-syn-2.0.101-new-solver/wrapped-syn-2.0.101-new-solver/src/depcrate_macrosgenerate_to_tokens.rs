// Generated macro for generate_to_tokens (macro)
macro_rules! Depcrate_macrosgenerate_to_tokens {
() => {
// Module: crate::macros
// Provides: {"generate_to_tokens"}
// Dependencies: {}
# [cfg (feature = "printing")] macro_rules ! generate_to_tokens { (($ ($ arms : tt) *) $ tokens : ident $ name : ident { $ (# [cfg $ cfg_attr : tt]) * $ (# [doc $ ($ doc_attr : tt) *]) * $ variant : ident , $ ($ next : tt) * }) => { generate_to_tokens ! (($ ($ arms) * $ (# [cfg $ cfg_attr]) * $ name ::$ variant => { }) $ tokens $ name { $ ($ next) * }) ; } ; (($ ($ arms : tt) *) $ tokens : ident $ name : ident { $ (# [cfg $ cfg_attr : tt]) * $ (# [doc $ ($ doc_attr : tt) *]) * $ variant : ident ($ member : ident) , $ ($ next : tt) * }) => { generate_to_tokens ! (($ ($ arms) * $ (# [cfg $ cfg_attr]) * $ name ::$ variant (_e) => _e . to_tokens ($ tokens) ,) $ tokens $ name { $ ($ next) * }) ; } ; (($ ($ arms : tt) *) $ tokens : ident $ name : ident { }) => { # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl :: quote :: ToTokens for $ name { fn to_tokens (& self , $ tokens : & mut :: proc_macro2 :: TokenStream) { match self { $ ($ arms) * } } } } ; }
};
}
