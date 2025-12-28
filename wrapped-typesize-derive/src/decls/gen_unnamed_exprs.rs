macro_rules! deps {
    () => {
        FieldConfig!();
    };
}

macro_rules! gen_unnamed_exprs {
    () => {
        deps!();
        fn gen_unnamed_exprs < 'a > (unnamed_fields : syn :: punctuated :: Iter < 'a , Field > , transform_unnamed : impl Fn (usize) -> TokenStream + 'a , common_body : impl Fn (TokenStream , TokenStream , FieldConfig) -> TokenStream + 'a ,) -> Option < impl ExactSizeIterator < Item = syn :: Result < TokenStream > > + 'a > { if unnamed_fields . len () == 0 { return None ; } let enumerated_iter = unnamed_fields . enumerate () ; Some (enumerated_iter . map (move | (i , field) | { let field_config = get_field_config (& field . attrs) ? ; Ok (common_body (transform_unnamed (i) , quote ! (# i) , field_config)) })) }
    };
}

gen_unnamed_exprs!()