// Generated macro for impl_162 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_162 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_162"}
// Dependencies: {}
impl RustcInternal for ProjectionElem { type T < 'tcx > = rustc_middle :: mir :: PlaceElem < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { ProjectionElem :: Deref => rustc_middle :: mir :: PlaceElem :: Deref , ProjectionElem :: Field (idx , ty) => { rustc_middle :: mir :: PlaceElem :: Field ((* idx) . into () , ty . internal (tables , tcx)) } ProjectionElem :: Index (idx) => rustc_middle :: mir :: PlaceElem :: Index ((* idx) . into ()) , ProjectionElem :: ConstantIndex { offset , min_length , from_end } => { rustc_middle :: mir :: PlaceElem :: ConstantIndex { offset : * offset , min_length : * min_length , from_end : * from_end , } } ProjectionElem :: Subslice { from , to , from_end } => { rustc_middle :: mir :: PlaceElem :: Subslice { from : * from , to : * to , from_end : * from_end } } ProjectionElem :: Downcast (idx) => { rustc_middle :: mir :: PlaceElem :: Downcast (None , idx . internal (tables , tcx)) } ProjectionElem :: OpaqueCast (ty) => { rustc_middle :: mir :: PlaceElem :: OpaqueCast (ty . internal (tables , tcx)) } ProjectionElem :: Subtype (ty) => { rustc_middle :: mir :: PlaceElem :: Subtype (ty . internal (tables , tcx)) } } } }
};
}
