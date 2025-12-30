// Generated macro for impl_58 (impl)
macro_rules! Depcrate_arbitraryimpl_58 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_58"}
// Dependencies: {}
impl UnaryFilter { fn make_let_expr (& self , var : & Ident , ps : & TokenStream , lets : & TokenStream) -> TokenStream { self . to_expr_filter () . make_let_as_expr (var , ps , lets) } fn make_let_fn (& self , var : & Ident) -> TokenStream { if self . arg_exists { let arg = & self . arg ; let arg_ty = & self . arg_ty ; let lets = if self . arg_by_ref { quote ! () } else { quote ! (let # arg = <# arg_ty as std :: clone :: Clone >:: clone (# arg) ;) } ; self . filter . make_let_as_expr (var , & quote ! (# arg) , & lets) } else { self . filter . make_let_as_fn (var , & self . arg_ty) } } fn to_expr_filter (& self) -> Filter { if self . arg_exists { self . filter . clone () } else { self . filter . with_fn_arg (& self . arg , self . arg_by_ref , & self . arg_ty) } } }
};
}
