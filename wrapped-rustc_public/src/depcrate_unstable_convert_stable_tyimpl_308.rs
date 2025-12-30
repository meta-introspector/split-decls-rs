// Generated macro for impl_308 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_308 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_308"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: SubtypePredicate < 'tcx > { type T = crate :: ty :: SubtypePredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: SubtypePredicate { a , b , a_is_expected : _ } = self ; crate :: ty :: SubtypePredicate { a : a . stable (tables , cx) , b : b . stable (tables , cx) } } }
};
}
