// Generated macro for impl_279 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_279 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_279"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AdtKind { type T = AdtKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: AdtKind :: Struct => AdtKind :: Struct , ty :: AdtKind :: Union => AdtKind :: Union , ty :: AdtKind :: Enum => AdtKind :: Enum , } } }
};
}
