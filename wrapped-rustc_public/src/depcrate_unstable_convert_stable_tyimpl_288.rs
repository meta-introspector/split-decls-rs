// Generated macro for impl_288 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_288 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_288"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: BoundVariableKind { type T = crate :: ty :: BoundVariableKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundVariableKind ; match self { ty :: BoundVariableKind :: Ty (bound_ty_kind) => { BoundVariableKind :: Ty (bound_ty_kind . stable (tables , cx)) } ty :: BoundVariableKind :: Region (bound_region_kind) => { BoundVariableKind :: Region (bound_region_kind . stable (tables , cx)) } ty :: BoundVariableKind :: Const => BoundVariableKind :: Const , } } }
};
}
