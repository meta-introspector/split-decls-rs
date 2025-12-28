macro_rules! with_lifetime_bound {
    () => {
        pub fn with_lifetime_bound (generics : & syn :: Generics , lifetime : & str) -> syn :: Generics { let bound = syn :: Lifetime :: new (lifetime , Span :: call_site ()) ; let def = syn :: LifetimeParam { attrs : Vec :: new () , lifetime : bound . clone () , colon_token : None , bounds : Punctuated :: new () , } ; let params = Some (syn :: GenericParam :: Lifetime (def)) . into_iter () . chain (generics . params . iter () . cloned () . map (| mut param | { match & mut param { syn :: GenericParam :: Lifetime (param) => { param . bounds . push (bound . clone ()) ; } syn :: GenericParam :: Type (param) => { param . bounds . push (syn :: TypeParamBound :: Lifetime (bound . clone ())) ; } syn :: GenericParam :: Const (_) => { } } param })) . collect () ; syn :: Generics { params , .. generics . clone () } }
    };
}

with_lifetime_bound!();