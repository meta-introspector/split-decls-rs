// Generated macro for impl_282 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_282 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: GenericArgKind < 'tcx > { type T = crate :: ty :: GenericArgKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: GenericArgKind ; match self { ty :: GenericArgKind :: Lifetime (region) => { GenericArgKind :: Lifetime (region . stable (tables , cx)) } ty :: GenericArgKind :: Type (ty) => GenericArgKind :: Type (ty . stable (tables , cx)) , ty :: GenericArgKind :: Const (cnst) => GenericArgKind :: Const (cnst . stable (tables , cx)) , } } }
};
}
