// Generated macro for with_self_bound (function)
macro_rules! Depcrate_boundwith_self_bound {
() => {
// Module: crate::bound
// Provides: {"with_self_bound"}
// Dependencies: {}
pub fn with_self_bound (cont : & Container , generics : & syn :: Generics , bound : & syn :: Path ,) -> syn :: Generics { let mut generics = generics . clone () ; generics . make_where_clause () . predicates . push (syn :: WherePredicate :: Type (syn :: PredicateType { lifetimes : None , bounded_ty : type_of_item (cont) , colon_token : < Token ! [:] > :: default () , bounds : vec ! [syn :: TypeParamBound :: Trait (syn :: TraitBound { paren_token : None , modifier : syn :: TraitBoundModifier :: None , lifetimes : None , path : bound . clone () , })] . into_iter () . collect () , })) ; generics }
};
}
