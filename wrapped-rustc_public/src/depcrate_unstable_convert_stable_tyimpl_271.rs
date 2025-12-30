// Generated macro for impl_271 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_271 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AliasTerm < 'tcx > { type T = crate :: ty :: AliasTerm ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: AliasTerm { args , def_id , .. } = self ; crate :: ty :: AliasTerm { def_id : tables . alias_def (* def_id) , args : args . stable (tables , cx) } } }
};
}
