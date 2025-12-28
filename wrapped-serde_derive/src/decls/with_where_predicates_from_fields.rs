macro_rules! deps {
    () => {
        Field!();
        Container!();
    };
}

macro_rules! with_where_predicates_from_fields {
    () => {
        deps!();
        pub fn with_where_predicates_from_fields (cont : & Container , generics : & syn :: Generics , from_field : fn (& attr :: Field) -> Option < & [syn :: WherePredicate] > ,) -> syn :: Generics { let predicates = cont . data . all_fields () . filter_map (| field | from_field (& field . attrs)) . flat_map (< [syn :: WherePredicate] > :: to_vec) ; let mut generics = generics . clone () ; generics . make_where_clause () . predicates . extend (predicates) ; generics }
    };
}

with_where_predicates_from_fields!()