macro_rules! deps {
    () => {
        FieldConfig!();
    };
}

macro_rules! gen_named_exprs {
    () => {
        deps!();
        fn gen_named_exprs < 'a > (named_fields : syn :: punctuated :: Iter < 'a , Field > , transform_named : impl Fn (& 'a Ident) -> TokenStream + 'a , common_body : impl Fn (TokenStream , TokenStream , FieldConfig) -> TokenStream + 'a ,) -> Option < impl ExactSizeIterator < Item = syn :: Result < TokenStream > > + 'a > { if named_fields . len () == 0 { return None ; } Some (named_fields . map (move | field | { let ident = field . ident . as_ref () . unwrap () ; let field_config = get_field_config (& field . attrs) ? ; Ok (common_body (transform_named (ident) , quote ! (# ident) , field_config ,)) })) }
    };
}

gen_named_exprs!()