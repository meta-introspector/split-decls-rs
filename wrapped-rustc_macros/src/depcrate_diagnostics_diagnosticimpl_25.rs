// Generated macro for impl_25 (impl)
macro_rules! Depcrate_diagnostics_diagnosticimpl_25 {
() => {
// Module: crate::diagnostics::diagnostic
// Provides: {"impl_25"}
// Dependencies: {}
impl Mismatch { # [doc = " Checks whether the slug starts with the crate name it's in."] fn check (slug : & syn :: Path) -> Option < Mismatch > { let crate_name = std :: env :: var ("CARGO_CRATE_NAME") . ok () ? ; let Some (("rustc" , slug_prefix)) = crate_name . split_once ('_') else { return None } ; let slug_name = slug . segments . first () ? . ident . to_string () ; if ! slug_name . starts_with (slug_prefix) { Some (Mismatch { slug_name , slug_prefix : slug_prefix . to_string () , crate_name }) } else { None } } }
};
}
