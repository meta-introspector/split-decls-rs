// Generated macro for impl_struct (function)
macro_rules! Depcrate_deriveimpl_struct {
() => {
// Module: crate::derive
// Provides: {"impl_struct"}
// Dependencies: {}
fn impl_struct (data : & syn :: DataStruct , input : & syn :: DeriveInput) -> syn :: Result < TokenStream2 > { let ident = & input . ident ; let tag = parse_tag_attr (input , & data . fields , & input . attrs , quote ! { # ident }) ? ; let into_arms = quote ! { _ => 0 , } ; let from_arms = quote ! { 0 => # tag , } ; let impl_trait = impl_trait (& input . ident , into_arms , from_arms) ; let declare_macro = tag . declare_macro () . unwrap_or_else (| | quote ! { }) ; Ok (quote ! { # impl_trait # declare_macro }) }
};
}
