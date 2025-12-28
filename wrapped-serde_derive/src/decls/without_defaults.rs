macro_rules! without_defaults {
    () => {
        pub fn without_defaults (generics : & syn :: Generics) -> syn :: Generics { syn :: Generics { params : generics . params . iter () . map (| param | match param { syn :: GenericParam :: Type (param) => syn :: GenericParam :: Type (syn :: TypeParam { eq_token : None , default : None , .. param . clone () }) , _ => param . clone () , }) . collect () , .. generics . clone () } }
    };
}

without_defaults!()