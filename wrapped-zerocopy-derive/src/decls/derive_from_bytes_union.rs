macro_rules! deps {
    () => {
        FieldBounds!();
        TraitBound!();
        ImplBlockBuilder!();
        Trait!();
    };
}

macro_rules! derive_from_bytes_union {
    () => {
        deps!();
        # [doc = " Unions are `FromBytes` if"] # [doc = " - all fields are `FromBytes` and `Immutable`"] fn derive_from_bytes_union (ast : & DeriveInput , unn : & DataUnion , zerocopy_crate : & Path ,) -> TokenStream { let field_type_trait_bounds = FieldBounds :: All (& [TraitBound :: Slf , TraitBound :: Other (Trait :: Immutable)]) ; ImplBlockBuilder :: new (ast , unn , Trait :: FromBytes , field_type_trait_bounds , zerocopy_crate) . build () }
    };
}

derive_from_bytes_union!()