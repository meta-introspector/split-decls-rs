// Generated macro for with_where_predicates (function)
macro_rules! Depcrate_boundwith_where_predicates {
() => {
// Module: crate::bound
// Provides: {"with_where_predicates"}
// Dependencies: {}
pub fn with_where_predicates (generics : & syn :: Generics , predicates : & [syn :: WherePredicate] ,) -> syn :: Generics { let mut generics = generics . clone () ; generics . make_where_clause () . predicates . extend (predicates . iter () . cloned ()) ; generics }
};
}
