// Generated macro for expr_for_struct (function)
macro_rules! Depcrate_arbitraryexpr_for_struct {
() => {
// Module: crate::arbitrary
// Provides: {"expr_for_struct"}
// Dependencies: {}
fn expr_for_struct (input : & DeriveInput , data : & DataStruct , bounds : & mut Bounds ,) -> Result < TokenStream > { let generics = GenericParamSet :: new (& input . generics) ; expr_for_fields (parse_quote ! (Self) , & generics , & data . fields , & input . attrs , true , bounds ,) }
};
}
