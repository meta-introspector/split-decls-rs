macro_rules! deps {
    () => {
        Container!();
        Data!();
        Variant!();
    };
}

macro_rules! with_where_predicates_from_variants {
    () => {
        deps!();
        pub fn with_where_predicates_from_variants (cont : & Container , generics : & syn :: Generics , from_variant : fn (& attr :: Variant) -> Option < & [syn :: WherePredicate] > ,) -> syn :: Generics { let variants = match & cont . data { Data :: Enum (variants) => variants , Data :: Struct (_ , _) => { return generics . clone () ; } } ; let predicates = variants . iter () . filter_map (| variant | from_variant (& variant . attrs)) . flat_map (< [syn :: WherePredicate] > :: to_vec) ; let mut generics = generics . clone () ; generics . make_where_clause () . predicates . extend (predicates) ; generics }
    };
}

with_where_predicates_from_variants!();