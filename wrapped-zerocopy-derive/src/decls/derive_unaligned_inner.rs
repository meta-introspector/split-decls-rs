macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! derive_unaligned_inner {
    () => {
        deps!();
        fn derive_unaligned_inner (ast : & DeriveInput , _top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { match & ast . data { Data :: Struct (strct) => derive_unaligned_struct (ast , strct , zerocopy_crate) , Data :: Enum (enm) => derive_unaligned_enum (ast , enm , zerocopy_crate) , Data :: Union (unn) => derive_unaligned_union (ast , unn , zerocopy_crate) , } }
    };
}

derive_unaligned_inner!()