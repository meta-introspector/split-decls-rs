// Generated macro for impl_230 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_230 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: RawPtrKind { type T = crate :: mir :: RawPtrKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use mir :: RawPtrKind :: * ; match * self { Const => crate :: mir :: RawPtrKind :: Const , Mut => crate :: mir :: RawPtrKind :: Mut , FakeForPtrMetadata => crate :: mir :: RawPtrKind :: FakeForPtrMetadata , } } }
};
}
