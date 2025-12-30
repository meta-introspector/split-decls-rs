// Generated macro for impl_419 (impl)
macro_rules! Depcrate_value_analysisimpl_419 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_419"}
// Dependencies: {}
impl < V , T > TryFrom < ProjectionElem < V , T > > for TrackElem { type Error = () ; fn try_from (value : ProjectionElem < V , T >) -> Result < Self , Self :: Error > { match value { ProjectionElem :: Field (field , _) => Ok (TrackElem :: Field (field)) , ProjectionElem :: Downcast (_ , idx) => Ok (TrackElem :: Variant (idx)) , _ => Err (()) , } } }
};
}
