// Generated macro for derive_zeroize_impl (function)
macro_rules! Depcratederive_zeroize_impl {
() => {
// Module: crate
// Provides: {"derive_zeroize_impl"}
// Dependencies: {}
fn derive_zeroize_impl (input : DeriveInput) -> TokenStream { let attributes = ZeroizeAttrs :: parse (& input) ; let mut generics = input . generics . clone () ; let extra_bounds = match attributes . bound { Some (bounds) => bounds . 0 , None => attributes . auto_params . iter () . map (| type_param | -> WherePredicate { parse_quote ! { # type_param : Zeroize } }) . collect () , } ; generics . make_where_clause () . predicates . extend (extra_bounds) ; let ty_name = & input . ident ; let (impl_gen , type_gen , where_) = generics . split_for_impl () ; let drop_impl = if attributes . drop { quote ! { # [doc (hidden)] impl # impl_gen Drop for # ty_name # type_gen # where_ { fn drop (& mut self) { self . zeroize () } } } } else { quote ! { } } ; let zeroizers = generate_fields (& input , quote ! { zeroize }) ; let zeroize_impl = quote ! { impl # impl_gen :: zeroize :: Zeroize for # ty_name # type_gen # where_ { fn zeroize (& mut self) { # zeroizers } } } ; quote ! { # zeroize_impl # drop_impl } }
};
}
