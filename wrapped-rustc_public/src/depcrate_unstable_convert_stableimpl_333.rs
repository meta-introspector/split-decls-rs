// Generated macro for impl_333 (impl)
macro_rules! Depcrate_unstable_convert_stableimpl_333 {
() => {
// Module: crate::unstable::convert::stable
// Provides: {"impl_333"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_span :: Span { type T = crate :: ty :: Span ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . create_span (* self) } }
};
}
