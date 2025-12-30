// Generated macro for pretend_used (function)
macro_rules! Depcrate_pretendpretend_used {
() => {
// Module: crate::pretend
// Provides: {"pretend_used"}
// Dependencies: {}
pub fn pretend_used (cont : & Container , is_packed : bool) -> TokenStream { let pretend_fields = pretend_fields_used (cont , is_packed) ; let pretend_variants = pretend_variants_used (cont) ; quote ! { # pretend_fields # pretend_variants } }
};
}
