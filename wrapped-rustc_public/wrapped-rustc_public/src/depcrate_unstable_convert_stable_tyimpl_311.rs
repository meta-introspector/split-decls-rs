// Generated macro for impl_311 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_311 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_311"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: TraitPredicate < 'tcx > { type T = crate :: ty :: TraitPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: TraitPredicate { trait_ref , polarity } = self ; crate :: ty :: TraitPredicate { trait_ref : trait_ref . stable (tables , cx) , polarity : polarity . stable (tables , cx) , } } }
};
}
