// Generated macro for impl_240 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_240 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_240"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: PlaceElem < 'tcx > { type T = crate :: mir :: ProjectionElem ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: ProjectionElem :: * ; match self { Deref => crate :: mir :: ProjectionElem :: Deref , Field (idx , ty) => { crate :: mir :: ProjectionElem :: Field (idx . stable (tables , cx) , ty . stable (tables , cx)) } Index (local) => crate :: mir :: ProjectionElem :: Index (local . stable (tables , cx)) , ConstantIndex { offset , min_length , from_end } => { crate :: mir :: ProjectionElem :: ConstantIndex { offset : * offset , min_length : * min_length , from_end : * from_end , } } Subslice { from , to , from_end } => { crate :: mir :: ProjectionElem :: Subslice { from : * from , to : * to , from_end : * from_end } } Downcast (_ , idx) => crate :: mir :: ProjectionElem :: Downcast (idx . stable (tables , cx)) , OpaqueCast (ty) => crate :: mir :: ProjectionElem :: OpaqueCast (ty . stable (tables , cx)) , Subtype (ty) => crate :: mir :: ProjectionElem :: Subtype (ty . stable (tables , cx)) , UnwrapUnsafeBinder (..) => todo ! ("FIXME(unsafe_binders):") , } } }
};
}
