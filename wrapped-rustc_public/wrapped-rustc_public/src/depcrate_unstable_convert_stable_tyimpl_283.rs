// Generated macro for impl_283 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_283 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'tcx , S , V > Stable < 'tcx > for ty :: Binder < 'tcx , S > where S : Stable < 'tcx , T = V > , { type T = crate :: ty :: Binder < V > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: Binder ; Binder { value : self . as_ref () . skip_binder () . stable (tables , cx) , bound_vars : self . bound_vars () . iter () . map (| bound_var | bound_var . stable (tables , cx)) . collect () , } } }
};
}
