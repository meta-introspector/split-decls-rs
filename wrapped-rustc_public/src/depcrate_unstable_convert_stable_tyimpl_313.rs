// Generated macro for impl_313 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_313 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_313"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ProjectionPredicate < 'tcx > { type T = crate :: ty :: ProjectionPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ProjectionPredicate { projection_term , term } = self ; crate :: ty :: ProjectionPredicate { projection_term : projection_term . stable (tables , cx) , term : term . kind () . stable (tables , cx) , } } }
};
}
