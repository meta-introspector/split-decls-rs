// Generated macro for impl_299 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_299 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_299"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: trait_def :: TraitSpecializationKind { type T = crate :: ty :: TraitSpecializationKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: TraitSpecializationKind ; match self { ty :: trait_def :: TraitSpecializationKind :: None => TraitSpecializationKind :: None , ty :: trait_def :: TraitSpecializationKind :: Marker => TraitSpecializationKind :: Marker , ty :: trait_def :: TraitSpecializationKind :: AlwaysApplicable => { TraitSpecializationKind :: AlwaysApplicable } } } }
};
}
