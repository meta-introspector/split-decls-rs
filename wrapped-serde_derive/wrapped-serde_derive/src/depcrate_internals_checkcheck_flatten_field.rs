// Generated macro for check_flatten_field (function)
macro_rules! Depcrate_internals_checkcheck_flatten_field {
() => {
// Module: crate::internals::check
// Provides: {"check_flatten_field"}
// Dependencies: {}
fn check_flatten_field (cx : & Ctxt , style : Style , field : & Field) { if ! field . attrs . flatten () { return ; } match style { Style :: Tuple => { cx . error_spanned_by (field . original , "#[serde(flatten)] cannot be used on tuple structs" ,) ; } Style :: Newtype => { cx . error_spanned_by (field . original , "#[serde(flatten)] cannot be used on newtype structs" ,) ; } _ => { } } }
};
}
