macro_rules! with_where_predicates {
    () => {
        pub fn with_where_predicates (generics : & syn :: Generics , predicates : & [syn :: WherePredicate] ,) -> syn :: Generics { let mut generics = generics . clone () ; generics . make_where_clause () . predicates . extend (predicates . iter () . cloned ()) ; generics }
    };
}

with_where_predicates!();