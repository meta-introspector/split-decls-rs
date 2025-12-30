// Generated macro for impl_309 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_309 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_309"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: CoercePredicate < 'tcx > { type T = crate :: ty :: CoercePredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: CoercePredicate { a , b } = self ; crate :: ty :: CoercePredicate { a : a . stable (tables , cx) , b : b . stable (tables , cx) } } }
};
}
