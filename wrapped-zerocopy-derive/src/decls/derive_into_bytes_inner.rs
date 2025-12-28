macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! derive_into_bytes_inner {
    () => {
        deps!();
        fn derive_into_bytes_inner (ast : & DeriveInput , _top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { match & ast . data { Data :: Struct (strct) => derive_into_bytes_struct (ast , strct , zerocopy_crate) , Data :: Enum (enm) => derive_into_bytes_enum (ast , enm , zerocopy_crate) , Data :: Union (unn) => derive_into_bytes_union (ast , unn , zerocopy_crate) , } }
    };
}

derive_into_bytes_inner!()