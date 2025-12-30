// Generated macro for impl_170 (impl)
macro_rules! Depcrate_litimpl_170 {
() => {
// Module: crate::lit
// Provides: {"impl_170"}
// Dependencies: {}
impl Lit { pub fn into_token_tree (self) -> TokenTree { let kind = match self . value { LitKind :: Bool (true) => TokenNode :: Term (Term :: intern ("true")) , LitKind :: Bool (false) => TokenNode :: Term (Term :: intern ("false")) , LitKind :: Other (l) => TokenNode :: Literal (l) , } ; TokenTree (proc_macro2 :: TokenTree { span : self . span . 0 , kind : kind , }) } }
};
}
