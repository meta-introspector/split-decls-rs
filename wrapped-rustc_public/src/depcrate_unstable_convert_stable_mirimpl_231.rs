// Generated macro for impl_231 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_231 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: BorrowKind { type T = crate :: mir :: BorrowKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: BorrowKind :: * ; match * self { Shared => crate :: mir :: BorrowKind :: Shared , Fake (kind) => crate :: mir :: BorrowKind :: Fake (kind . stable (tables , cx)) , Mut { kind } => crate :: mir :: BorrowKind :: Mut { kind : kind . stable (tables , cx) } , } } }
};
}
