// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_spannedimpl_1319 {
() => {
// Module: crate::spanned
// Provides: {"impl_1319"}
// Dependencies: {}
impl Spanned for ast :: Arm { fn span (& self) -> Span { let lo = if self . attrs . is_empty () { self . pat . span . lo () } else { self . attrs [0] . span . lo () } ; let hi = if let Some (body) = & self . body { body . span . hi () } else { self . pat . span . hi () } ; span_with_attrs_lo_hi ! (self , lo , hi) } }
};
}
