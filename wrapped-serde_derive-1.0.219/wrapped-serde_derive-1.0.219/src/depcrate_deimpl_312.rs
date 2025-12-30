// Generated macro for impl_312 (impl)
macro_rules! Depcrate_deimpl_312 {
() => {
// Module: crate::de
// Provides: {"impl_312"}
// Dependencies: {}
# [cfg (feature = "deserialize_in_place")] impl < 'a > ToTokens for InPlaceTypeGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let mut generics = self . 0 . generics . clone () ; generics . params = Some (syn :: GenericParam :: Lifetime (place_lifetime ())) . into_iter () . chain (generics . params) . collect () ; de_type_generics_to_tokens (generics , & self . 0 . borrowed , tokens) ; } }
};
}
