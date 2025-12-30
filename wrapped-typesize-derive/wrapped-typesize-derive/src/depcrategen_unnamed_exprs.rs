// Generated macro for gen_unnamed_exprs (function)
macro_rules! Depcrategen_unnamed_exprs {
() => {
// Module: crate
// Provides: {"gen_unnamed_exprs"}
// Dependencies: {}
fn gen_unnamed_exprs < 'a > (unnamed_fields : syn :: punctuated :: Iter < 'a , Field > , transform_unnamed : impl Fn (usize) -> TokenStream + 'a , common_body : impl Fn (TokenStream , TokenStream , FieldConfig) -> TokenStream + 'a ,) -> Option < impl ExactSizeIterator < Item = syn :: Result < TokenStream > > + 'a > { if unnamed_fields . len () == 0 { return None ; } let enumerated_iter = unnamed_fields . enumerate () ; Some (enumerated_iter . map (move | (i , field) | { let field_config = get_field_config (& field . attrs) ? ; Ok (common_body (transform_unnamed (i) , quote ! (# i) , field_config)) })) }
};
}
