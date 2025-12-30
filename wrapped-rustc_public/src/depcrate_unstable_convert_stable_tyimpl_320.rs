// Generated macro for impl_320 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_320 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_320"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Movability { type T = crate :: ty :: Movability ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Movability :: Static => crate :: ty :: Movability :: Static , ty :: Movability :: Movable => crate :: ty :: Movability :: Movable , } } }
};
}
