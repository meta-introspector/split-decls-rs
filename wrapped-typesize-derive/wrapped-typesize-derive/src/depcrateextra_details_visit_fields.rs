// Generated macro for extra_details_visit_fields (function)
macro_rules! Depcrateextra_details_visit_fields {
() => {
// Module: crate
// Provides: {"extra_details_visit_fields"}
// Dependencies: {}
fn extra_details_visit_fields < 'a > (fields : & 'a syn :: Fields , transform_named : impl Fn (& 'a Ident) -> TokenStream + 'a , transform_unnamed : impl Fn (usize) -> TokenStream + 'a , pass_mode : PassMode ,) -> syn :: Result < TokenStream > { for_each_field (fields , Punct :: new ('+' , Spacing :: Alone) , transform_named , transform_unnamed , move | ident , _name , config | match config { FieldConfig :: Skip => quote ! (0) , FieldConfig :: Default => { gen_call_with_arg (& quote ! (:: typesize :: TypeSize :: extra_size) , & ident , pass_mode) } FieldConfig :: With (fn_path) => { gen_call_with_arg (& fn_path . into_token_stream () , & ident , pass_mode) } } ,) . unwrap_or_else (| | Ok (quote ! (0_usize))) }
};
}
