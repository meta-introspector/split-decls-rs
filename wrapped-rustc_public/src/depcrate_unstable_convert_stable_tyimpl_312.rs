// Generated macro for impl_312 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_312 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_312"}
// Dependencies: {}
impl < 'tcx , T > Stable < 'tcx > for ty :: OutlivesPredicate < 'tcx , T > where T : Stable < 'tcx > , { type T = crate :: ty :: OutlivesPredicate < T :: T , Region > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: OutlivesPredicate (a , b) = self ; crate :: ty :: OutlivesPredicate (a . stable (tables , cx) , b . stable (tables , cx)) } }
};
}
