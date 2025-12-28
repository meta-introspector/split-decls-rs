macro_rules! deps {
    () => {
        InPlaceTypeGenerics!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] impl < 'a > ToTokens for InPlaceTypeGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let mut generics = self . 0 . generics . clone () ; generics . params = Some (syn :: GenericParam :: Lifetime (place_lifetime ())) . into_iter () . chain (generics . params) . collect () ; de_type_generics_to_tokens (generics , & self . 0 . borrowed , tokens) ; } }
    };
}

impl_224!()