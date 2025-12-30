// Generated macro for for_each_field (function)
macro_rules! Depcratefor_each_field {
() => {
// Module: crate
// Provides: {"for_each_field"}
// Dependencies: {}
fn for_each_field < 'a > (fields : & 'a syn :: Fields , join_with : Punct , transform_named : impl Fn (& 'a Ident) -> TokenStream + 'a , transform_unnamed : impl Fn (usize) -> TokenStream + 'a , common_body : impl Fn (TokenStream , TokenStream , FieldConfig) -> TokenStream + 'a ,) -> Option < syn :: Result < TokenStream > > { match fields { syn :: Fields :: Named (fields) => Some (try_join_tokens (gen_named_exprs (fields . named . iter () , transform_named , common_body) ? , join_with ,)) , syn :: Fields :: Unnamed (fields) => Some (try_join_tokens (gen_unnamed_exprs (fields . unnamed . iter () , transform_unnamed , common_body) ? , join_with ,)) , syn :: Fields :: Unit => None , } }
};
}
