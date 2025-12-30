// Generated macro for impl_291 (impl)
macro_rules! Depcrate_deimpl_291 {
() => {
// Module: crate::de
// Provides: {"impl_291"}
// Dependencies: {}
impl < 'a > ToTokens for DeImplGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let mut generics = self . 0 . generics . clone () ; if let Some (de_lifetime) = self . 0 . borrowed . de_lifetime_param () { generics . params = Some (syn :: GenericParam :: Lifetime (de_lifetime)) . into_iter () . chain (generics . params) . collect () ; } let (impl_generics , _ , _) = generics . split_for_impl () ; impl_generics . to_tokens (tokens) ; } }
};
}
