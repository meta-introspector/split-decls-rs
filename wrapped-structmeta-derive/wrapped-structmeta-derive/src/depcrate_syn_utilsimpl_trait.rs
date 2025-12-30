// Generated macro for impl_trait (function)
macro_rules! Depcrate_syn_utilsimpl_trait {
() => {
// Module: crate::syn_utils
// Provides: {"impl_trait"}
// Dependencies: {}
pub fn impl_trait (input : & DeriveInput , trait_path : & Path , wheres : & [WherePredicate] , contents : TokenStream ,) -> TokenStream { let ty = & input . ident ; let (impl_g , ty_g , where_clause) = input . generics . split_for_impl () ; let mut wheres = wheres . to_vec () ; if let Some (where_clause) = where_clause { wheres . extend (where_clause . predicates . iter () . cloned ()) ; } let where_clause = if wheres . is_empty () { quote ! { } } else { quote ! { where # (# wheres ,) * } } ; quote ! { # [automatically_derived] impl # impl_g # trait_path for # ty # ty_g # where_clause { # contents } } }
};
}
