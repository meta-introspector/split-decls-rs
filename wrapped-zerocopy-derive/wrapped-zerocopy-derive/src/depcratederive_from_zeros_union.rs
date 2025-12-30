// Generated macro for derive_from_zeros_union (function)
macro_rules! Depcratederive_from_zeros_union {
() => {
// Module: crate
// Provides: {"derive_from_zeros_union"}
// Dependencies: {}
# [doc = " Unions are `FromZeros` if"] # [doc = " - all fields are `FromZeros` and `Immutable`"] fn derive_from_zeros_union (ast : & DeriveInput , unn : & DataUnion , zerocopy_crate : & Path ,) -> TokenStream { let field_type_trait_bounds = FieldBounds :: All (& [TraitBound :: Slf , TraitBound :: Other (Trait :: Immutable)]) ; ImplBlockBuilder :: new (ast , unn , Trait :: FromZeros , field_type_trait_bounds , zerocopy_crate) . build () }
};
}
