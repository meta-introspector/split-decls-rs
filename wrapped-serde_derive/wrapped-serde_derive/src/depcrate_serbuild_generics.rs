// Generated macro for build_generics (function)
macro_rules! Depcrate_serbuild_generics {
() => {
// Module: crate::ser
// Provides: {"build_generics"}
// Dependencies: {}
fn build_generics (cont : & Container) -> syn :: Generics { let generics = bound :: without_defaults (cont . generics) ; let generics = bound :: with_where_predicates_from_fields (cont , & generics , attr :: Field :: ser_bound) ; let generics = bound :: with_where_predicates_from_variants (cont , & generics , attr :: Variant :: ser_bound) ; match cont . attrs . ser_bound () { Some (predicates) => bound :: with_where_predicates (& generics , predicates) , None => bound :: with_bound (cont , & generics , needs_serialize_bound , & parse_quote ! (_serde :: Serialize) ,) , } }
};
}
