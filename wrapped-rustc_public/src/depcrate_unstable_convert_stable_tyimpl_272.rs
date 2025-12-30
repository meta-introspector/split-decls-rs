// Generated macro for impl_272 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_272 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: DynKind { type T = crate :: ty :: DynKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Dyn => crate :: ty :: DynKind :: Dyn , } } }
};
}
