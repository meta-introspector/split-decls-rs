// Generated macro for impl_286 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_286 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: BoundTyKind { type T = crate :: ty :: BoundTyKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundTyKind ; match self { ty :: BoundTyKind :: Anon => BoundTyKind :: Anon , ty :: BoundTyKind :: Param (def_id) => { BoundTyKind :: Param (tables . param_def (* def_id) , cx . tcx . item_name (* def_id) . to_string ()) } } } }
};
}
