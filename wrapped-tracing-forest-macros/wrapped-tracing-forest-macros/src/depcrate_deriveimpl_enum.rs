// Generated macro for impl_enum (function)
macro_rules! Depcrate_deriveimpl_enum {
() => {
// Module: crate::derive
// Provides: {"impl_enum"}
// Dependencies: {}
fn impl_enum (data : & syn :: DataEnum , input : & syn :: DeriveInput) -> syn :: Result < TokenStream2 > { let ident = & input . ident ; let tags = data . variants . iter () . map (| variant | { let var_ident = & variant . ident ; parse_tag_attr (variant , & variant . fields , & variant . attrs , quote ! { # ident ::# var_ident } ,) }) . collect :: < syn :: Result < Vec < TagRepr > > > () ? ; let len = data . variants . len () ; let variant_names = data . variants . iter () . map (| v | & v . ident) ; let ids = 0 .. len as u64 ; let into_arms = quote ! { # (Self ::# variant_names => # ids ,) * } ; let ids = 0 .. len as u64 ; let from_arms = quote ! { # (# ids => # tags ,) * } ; let impl_trait = impl_trait (& input . ident , into_arms , from_arms) ; let declare_macros = tags . iter () . filter_map (TagRepr :: declare_macro) ; Ok (quote ! { # impl_trait # (# declare_macros) * }) }
};
}
