// Generated macro for derive_from_zeros_struct (function)
macro_rules! Depcratederive_from_zeros_struct {
() => {
// Module: crate
// Provides: {"derive_from_zeros_struct"}
// Dependencies: {}
# [doc = " A struct is `FromZeros` if:"] # [doc = " - all fields are `FromZeros`"] fn derive_from_zeros_struct (ast : & DeriveInput , strct : & DataStruct , zerocopy_crate : & Path ,) -> TokenStream { ImplBlockBuilder :: new (ast , strct , Trait :: FromZeros , FieldBounds :: ALL_SELF , zerocopy_crate) . build () }
};
}
