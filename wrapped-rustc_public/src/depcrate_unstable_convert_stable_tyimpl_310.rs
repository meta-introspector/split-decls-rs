// Generated macro for impl_310 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_310 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_310"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AliasRelationDirection { type T = crate :: ty :: AliasRelationDirection ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: AliasRelationDirection :: * ; match self { Equate => crate :: ty :: AliasRelationDirection :: Equate , Subtype => crate :: ty :: AliasRelationDirection :: Subtype , } } }
};
}
