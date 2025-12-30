// Generated macro for try_from_str (function)
macro_rules! Depcrate_macros_strings_from_stringtry_from_str {
() => {
// Module: crate::macros::strings::from_string
// Provides: {"try_from_str"}
// Dependencies: {}
fn try_from_str (name : & proc_macro2 :: Ident , impl_generics : & syn :: ImplGenerics , ty_generics : & syn :: TypeGenerics , where_clause : Option < & syn :: WhereClause > , default_err_ty : & TokenStream ,) -> TokenStream { quote ! { # [allow (clippy :: use_self)] # [automatically_derived] impl # impl_generics :: core :: convert :: TryFrom <& str > for # name # ty_generics # where_clause { type Error = # default_err_ty ; # [inline] fn try_from (s : & str) -> :: core :: result :: Result < # name # ty_generics , < Self as :: core :: convert :: TryFrom <& str >>:: Error > { :: core :: str :: FromStr :: from_str (s) } } } }
};
}
