// Generated macro for SpannedValue (trait)
macro_rules! Depcrate_format_description_parseSpannedValue {
() => {
// Module: crate::format_description::parse
// Provides: {"SpannedValue"}
// Dependencies: {}
# [doc = " Helper trait to attach a [`Span`] to a value."] trait SpannedValue : Sized { # [doc = " Attach a [`Span`] to a value."] fn spanned (self , span : Span) -> Spanned < Self > ; }
};
}
