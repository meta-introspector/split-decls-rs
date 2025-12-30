// Generated macro for impl_54 (impl)
macro_rules! Depcrate_struct_metaimpl_54 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_54"}
// Dependencies: {}
impl UnnamedParam < '_ > { fn build_arm_parse_value (& self , index : usize) -> TokenStream { let temp_ident = & self . info . temp_ident ; let span = self . info . field . span () ; let expr = build_parse_expr (self . ty , span) ; quote_spanned ! { span => # index => { # temp_ident = Some (# expr) ; } } } fn build_arm_parse_vec_item (& self) -> TokenStream { let temp_ident = & self . info . temp_ident ; let span = self . info . field . span () ; let expr = build_parse_expr (self . ty , span) ; quote_spanned ! { span => _ => { # temp_ident . push (# expr) ; } } } fn build_ctor_arg (& self , var_is_option : bool , ctor_args : & mut [TokenStream]) { let temp_ident = & self . info . temp_ident ; let value = match (var_is_option , self . is_option) { (false , false) | (true , true) => { quote ! (# temp_ident) } (true , false) => { quote ! (# temp_ident . unwrap ()) } _ => { unreachable ! () } } ; build_ctor_arg (& self . info , value , ctor_args) } }
};
}
