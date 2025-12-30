// Generated macro for get_arms (function)
macro_rules! Depcrate_macros_strings_as_ref_strget_arms {
() => {
// Module: crate::macros::strings::as_ref_str
// Provides: {"get_arms"}
// Dependencies: {}
fn get_arms < F > (ast : & DeriveInput , transparent_fn : F) -> syn :: Result < Vec < TokenStream > > where F : Fn (& TokenStream) -> TokenStream , { let name = & ast . ident ; let mut arms = Vec :: new () ; let variants = match & ast . data { Data :: Enum (v) => & v . variants , _ => return Err (non_enum_error ()) , } ; let type_properties = ast . get_type_properties () ? ; for variant in variants { let ident = & variant . ident ; let variant_properties = variant . get_variant_properties () ? ; if variant_properties . disabled . is_some () { continue ; } if let Some (..) = variant_properties . transparent { let arm = super :: extract_single_field_variant_and_then (name , variant , | tok | { transparent_fn (tok) }) . map_err (| _ | non_single_field_variant_error ("transparent")) ? ; arms . push (arm) ; continue ; } let output = variant_properties . get_preferred_name (type_properties . case_style , type_properties . prefix . as_ref () , type_properties . suffix . as_ref () ,) ; let params = match variant . fields { Fields :: Unit => quote ! { } , Fields :: Unnamed (..) => quote ! { (..) } , Fields :: Named (..) => quote ! { { .. } } , } ; arms . push (quote ! { # name ::# ident # params => # output }) ; } if arms . len () < variants . len () { arms . push (quote ! { _ => panic ! ("AsRef::<str>::as_ref() or AsStaticRef::<str>::as_static() \
                 called on disabled variant." ,) }) ; } Ok (arms) }
};
}
