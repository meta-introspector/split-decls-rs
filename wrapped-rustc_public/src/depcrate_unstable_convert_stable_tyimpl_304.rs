// Generated macro for impl_304 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_304 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_304"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: GenericParamDef { type T = crate :: ty :: GenericParamDef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { GenericParamDef { name : self . name . to_string () , def_id : tables . generic_def (self . def_id) , index : self . index , pure_wrt_drop : self . pure_wrt_drop , kind : self . kind . stable (tables , cx) , } } }
};
}
