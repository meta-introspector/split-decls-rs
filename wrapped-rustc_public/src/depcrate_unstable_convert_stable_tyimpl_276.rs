// Generated macro for impl_276 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_276 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ExistentialProjection < 'tcx > { type T = crate :: ty :: ExistentialProjection ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ExistentialProjection { def_id , args , term , .. } = self ; crate :: ty :: ExistentialProjection { def_id : tables . trait_def (* def_id) , generic_args : args . stable (tables , cx) , term : term . kind () . stable (tables , cx) , } } }
};
}
