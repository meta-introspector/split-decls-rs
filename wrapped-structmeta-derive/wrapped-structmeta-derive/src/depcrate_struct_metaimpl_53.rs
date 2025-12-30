// Generated macro for impl_53 (impl)
macro_rules! Depcrate_struct_metaimpl_53 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_53"}
// Dependencies: {}
impl RestParam < '_ > { fn build_let (& self) -> TokenStream { let temp_ident = & self . info . temp_ident ; quote ! (let mut # temp_ident = :: std :: collections :: HashMap :: new () ;) } fn build_arm_parse (& self , kind : ArgKind) -> TokenStream { let temp_ident = & self . info . temp_ident ; let span = self . info . field . span () ; let expr = self . ty . build_parse_expr (kind , span) ; let var = kind . to_helper_name_index_variant () ; quote_spanned ! { span => :: structmeta :: helpers :: NameIndex ::# var (Err (name)) => { if # temp_ident . insert (name . to_string () , # expr) . is_some () { return Err (:: structmeta :: helpers :: exports :: syn :: Error :: new (span , format ! ("parameter `{}` specified more than once" , name))) ; } } } } fn build_ctor_arg (& self , ctor_args : & mut [TokenStream]) { let temp_ident = & self . info . temp_ident ; build_ctor_arg (& self . info , quote ! (# temp_ident) , ctor_args) } }
};
}
