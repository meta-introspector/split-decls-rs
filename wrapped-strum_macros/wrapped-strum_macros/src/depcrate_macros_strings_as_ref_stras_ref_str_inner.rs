// Generated macro for as_ref_str_inner (function)
macro_rules! Depcrate_macros_strings_as_ref_stras_ref_str_inner {
() => {
// Module: crate::macros::strings::as_ref_str
// Provides: {"as_ref_str_inner"}
// Dependencies: {}
pub fn as_ref_str_inner (ast : & DeriveInput) -> syn :: Result < TokenStream > { let name = & ast . ident ; let (impl_generics , ty_generics , where_clause) = ast . generics . split_for_impl () ; let arms = get_arms (ast , | tok | { quote ! { :: core :: convert :: AsRef ::< str >:: as_ref (# tok) } }) ? ; Ok (quote ! { # [automatically_derived] impl # impl_generics :: core :: convert :: AsRef < str > for # name # ty_generics # where_clause { # [inline] fn as_ref (& self) -> & str { match * self { # (# arms) ,* } } } }) }
};
}
