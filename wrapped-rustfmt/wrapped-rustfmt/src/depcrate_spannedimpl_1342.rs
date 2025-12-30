// Generated macro for impl_1342 (impl)
macro_rules! Depcrate_spannedimpl_1342 {
() => {
// Module: crate::spanned
// Provides: {"impl_1342"}
// Dependencies: {}
impl Spanned for ast :: Param { fn span (& self) -> Span { if crate :: items :: is_named_param (self) { mk_sp (crate :: items :: span_lo_for_param (self) , self . ty . span . hi ()) } else { self . ty . span } } }
};
}
