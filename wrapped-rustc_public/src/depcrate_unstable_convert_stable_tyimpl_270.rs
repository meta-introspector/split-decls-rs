// Generated macro for impl_270 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_270 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AliasTy < 'tcx > { type T = crate :: ty :: AliasTy ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: AliasTy { args , def_id , .. } = self ; crate :: ty :: AliasTy { def_id : tables . alias_def (* def_id) , args : args . stable (tables , cx) } } }
};
}
