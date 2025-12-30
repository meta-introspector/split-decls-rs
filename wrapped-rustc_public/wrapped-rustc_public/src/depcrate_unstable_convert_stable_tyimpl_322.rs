// Generated macro for impl_322 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_322 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_session :: cstore :: ForeignModule { type T = crate :: ty :: ForeignModule ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: ForeignModule { def_id : tables . foreign_module_def (self . def_id) , abi : self . abi . stable (tables , cx) , } } }
};
}
