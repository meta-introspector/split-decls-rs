// Generated macro for expr_for_fields (function)
macro_rules! Depcrate_arbitraryexpr_for_fields {
() => {
// Module: crate::arbitrary
// Provides: {"expr_for_fields"}
// Dependencies: {}
fn expr_for_fields (self_path : Path , generics : & GenericParamSet , fields : & Fields , attrs : & [Attribute] , filter_allow_fn : bool , bounds : & mut Bounds ,) -> Result < TokenStream > { let b = StrategyBuilder :: from_fields (self_path , fields , attrs , filter_allow_fn) ? ; b . get_bound_types (generics , bounds) ? ; b . build () }
};
}
