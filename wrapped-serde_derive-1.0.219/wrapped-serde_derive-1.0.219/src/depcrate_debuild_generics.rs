// Generated macro for build_generics (function)
macro_rules! Depcrate_debuild_generics {
() => {
// Module: crate::de
// Provides: {"build_generics"}
// Dependencies: {}
fn build_generics (cont : & Container , borrowed : & BorrowedLifetimes) -> syn :: Generics { let generics = bound :: without_defaults (cont . generics) ; let generics = bound :: with_where_predicates_from_fields (cont , & generics , attr :: Field :: de_bound) ; let generics = bound :: with_where_predicates_from_variants (cont , & generics , attr :: Variant :: de_bound) ; match cont . attrs . de_bound () { Some (predicates) => bound :: with_where_predicates (& generics , predicates) , None => { let generics = match * cont . attrs . default () { attr :: Default :: Default => bound :: with_self_bound (cont , & generics , & parse_quote ! (_serde :: __private :: Default) ,) , attr :: Default :: None | attr :: Default :: Path (_) => generics , } ; let delife = borrowed . de_lifetime () ; let generics = bound :: with_bound (cont , & generics , needs_deserialize_bound , & parse_quote ! (_serde :: Deserialize <# delife >) ,) ; bound :: with_bound (cont , & generics , requires_default , & parse_quote ! (_serde :: __private :: Default) ,) } } }
};
}
