macro_rules! deps {
    () => {
        InPlaceImplGenerics!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] impl < 'a > ToTokens for InPlaceImplGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let place_lifetime = place_lifetime () ; let mut generics = self . 0 . generics . clone () ; for param in & mut generics . params { match param { syn :: GenericParam :: Lifetime (param) => { param . bounds . push (place_lifetime . lifetime . clone ()) ; } syn :: GenericParam :: Type (param) => { param . bounds . push (syn :: TypeParamBound :: Lifetime (place_lifetime . lifetime . clone () ,)) ; } syn :: GenericParam :: Const (_) => { } } } generics . params = Some (syn :: GenericParam :: Lifetime (place_lifetime)) . into_iter () . chain (generics . params) . collect () ; if let Some (de_lifetime) = self . 0 . borrowed . de_lifetime_param () { generics . params = Some (syn :: GenericParam :: Lifetime (de_lifetime)) . into_iter () . chain (generics . params) . collect () ; } let (impl_generics , _ , _) = generics . split_for_impl () ; impl_generics . to_tokens (tokens) ; } }
    };
}

impl_218!()