// Generated macro for impl_274 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_274 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ExistentialTraitRef < 'tcx > { type T = crate :: ty :: ExistentialTraitRef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ExistentialTraitRef { def_id , args , .. } = self ; crate :: ty :: ExistentialTraitRef { def_id : tables . trait_def (* def_id) , generic_args : args . stable (tables , cx) , } } }
};
}
