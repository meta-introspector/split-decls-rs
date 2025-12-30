// Generated macro for impl_147 (impl)
macro_rules! Depcrate_intrinsicimpl_147 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_147"}
// Dependencies: {}
impl ToTokens for LLVMLinkAttribute { fn to_tokens (& self , tokens : & mut TokenStream) { let LLVMLinkAttribute { arch , link } = self ; let link = link . to_string () ; let mut cfg_attr_cond = TokenStream :: new () ; let mut single_arch = true ; for arch in arch . split (',') { if ! cfg_attr_cond . is_empty () { single_arch = false ; cfg_attr_cond . append (Punct :: new (',' , Spacing :: Alone)) ; } cfg_attr_cond . append_all (quote ! { target_arch = # arch }) ; } assert ! (! cfg_attr_cond . is_empty ()) ; if ! single_arch { cfg_attr_cond = quote ! { any (# cfg_attr_cond) } ; } tokens . append_all (quote ! { # [cfg_attr (# cfg_attr_cond , link_name = # link)] }) } }
};
}
