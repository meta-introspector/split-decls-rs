// Generated macro for impl_1320 (impl)
macro_rules! Depcrate_spannedimpl_1320 {
() => {
// Module: crate::spanned
// Provides: {"impl_1320"}
// Dependencies: {}
impl Spanned for ast :: Param { fn span (& self) -> Span { if crate :: items :: is_named_param (self) { mk_sp (crate :: items :: span_lo_for_param (self) , self . ty . span . hi ()) } else { self . ty . span } } }
};
}
