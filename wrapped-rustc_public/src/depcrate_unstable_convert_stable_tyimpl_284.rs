// Generated macro for impl_284 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_284 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'tcx , S , V > Stable < 'tcx > for ty :: EarlyBinder < 'tcx , S > where S : Stable < 'tcx , T = V > , { type T = crate :: ty :: EarlyBinder < V > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: EarlyBinder ; EarlyBinder { value : self . as_ref () . skip_binder () . stable (tables , cx) } } }
};
}
