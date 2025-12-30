// Generated macro for to_pattern (function)
macro_rules! Depcrate_to_tokensto_pattern {
() => {
// Module: crate::to_tokens
// Provides: {"to_pattern"}
// Dependencies: {}
fn to_pattern (self_path : TokenStream , fields : & Fields) -> TokenStream { let mut vars = Vec :: new () ; match fields { Fields :: Unit => self_path , Fields :: Unnamed (_) => { for (index , field) in fields . iter () . enumerate () { let var_ident = to_var_ident (Some (index) , & field . ident) ; vars . push (quote ! (# var_ident)) ; } quote ! (# self_path (# (# vars ,) *)) } Fields :: Named (_) => { for field in fields . iter () { let field_ident = & field . ident ; let var_ident = to_var_ident (None , field_ident) ; vars . push (quote ! (# field_ident : # var_ident)) ; } quote ! (# self_path { # (# vars ,) * }) } } }
};
}
