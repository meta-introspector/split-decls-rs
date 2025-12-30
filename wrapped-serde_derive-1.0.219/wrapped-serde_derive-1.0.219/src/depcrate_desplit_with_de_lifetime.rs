// Generated macro for split_with_de_lifetime (function)
macro_rules! Depcrate_desplit_with_de_lifetime {
() => {
// Module: crate::de
// Provides: {"split_with_de_lifetime"}
// Dependencies: {}
fn split_with_de_lifetime (params : & Parameters ,) -> (DeImplGenerics , DeTypeGenerics , syn :: TypeGenerics , Option < & syn :: WhereClause > ,) { let de_impl_generics = DeImplGenerics (params) ; let de_ty_generics = DeTypeGenerics (params) ; let (_ , ty_generics , where_clause) = params . generics . split_for_impl () ; (de_impl_generics , de_ty_generics , ty_generics , where_clause) }
};
}
