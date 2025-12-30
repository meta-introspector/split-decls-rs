// Generated macro for impl_275 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_275 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: TermKind < 'tcx > { type T = crate :: ty :: TermKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: TermKind ; match self { ty :: TermKind :: Ty (ty) => TermKind :: Type (ty . stable (tables , cx)) , ty :: TermKind :: Const (cnst) => { let cnst = cnst . stable (tables , cx) ; TermKind :: Const (cnst) } } } }
};
}
