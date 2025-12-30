// Generated macro for impl_48 (impl)
macro_rules! Depcrate_struct_metaimpl_48 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > ParamInfo < 'a > { fn new (index : usize , field : & 'a Field , ty : & 'a Type) -> Self { let temp_ident = format_ident ! ("_value_{}" , index) ; Self { index , field , ty , temp_ident , } } fn span (& self) -> Span { self . field . span () } fn build_let_none (& self) -> TokenStream { let temp_ident = & self . temp_ident ; let ty = & self . ty ; quote ! (let mut # temp_ident : Option <# ty > = None ;) } fn build_let_vec_new (& self) -> TokenStream { let temp_ident = & self . temp_ident ; let ty = & self . ty ; quote ! (let mut # temp_ident = <# ty >:: new () ;) } fn build_let_parse (& self) -> TokenStream { let temp_ident = & self . temp_ident ; let ty = & self . field . ty ; quote_spanned ! (self . span () => let # temp_ident = input . parse ::<# ty > () ?;) } }
};
}
