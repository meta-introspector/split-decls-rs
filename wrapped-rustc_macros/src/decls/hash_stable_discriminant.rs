macro_rules! hash_stable_discriminant {
    () => {
        fn hash_stable_discriminant (s : & mut synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { match s . ast () . data { syn :: Data :: Enum (_) => quote ! { :: std :: mem :: discriminant (self) . hash_stable (__hcx , __hasher) ; } , syn :: Data :: Struct (_) => quote ! { } , syn :: Data :: Union (_) => panic ! ("cannot derive on union") , } }
    };
}

hash_stable_discriminant!();