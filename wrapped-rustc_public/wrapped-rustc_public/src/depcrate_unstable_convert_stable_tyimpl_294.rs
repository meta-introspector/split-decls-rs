// Generated macro for impl_294 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_294 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_294"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Pattern < 'tcx > { type T = crate :: ty :: Pattern ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match * * self { ty :: PatternKind :: Range { start , end } => crate :: ty :: Pattern :: Range { start : Some (start . stable (tables , cx)) , end : Some (end . stable (tables , cx)) , include_end : true , } , ty :: PatternKind :: Or (_) => todo ! () , } } }
};
}
