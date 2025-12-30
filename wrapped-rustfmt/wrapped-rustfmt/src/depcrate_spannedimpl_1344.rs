// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_spannedimpl_1344 {
() => {
// Module: crate::spanned
// Provides: {"impl_1344"}
// Dependencies: {}
impl Spanned for ast :: FieldDef { fn span (& self) -> Span { span_with_attrs_lo_hi ! (self , self . span . lo () , self . ty . span . hi ()) } }
};
}
