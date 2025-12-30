// Generated macro for impl_292 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_292 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_292"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for Ty < 'tcx > { type T = crate :: ty :: Ty ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . intern_ty (cx . lift (* self) . unwrap ()) } }
};
}
