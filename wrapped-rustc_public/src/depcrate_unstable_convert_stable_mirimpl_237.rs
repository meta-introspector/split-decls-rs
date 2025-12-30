// Generated macro for impl_237 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_237 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Operand < 'tcx > { type T = crate :: mir :: Operand ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: Operand :: * ; match self { Copy (place) => crate :: mir :: Operand :: Copy (place . stable (tables , cx)) , Move (place) => crate :: mir :: Operand :: Move (place . stable (tables , cx)) , Constant (c) => crate :: mir :: Operand :: Constant (c . stable (tables , cx)) , } } }
};
}
