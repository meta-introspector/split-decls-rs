// Generated macro for PlaceCollector (struct)
macro_rules! Depcrate_value_analysisPlaceCollector {
() => {
// Module: crate::value_analysis
// Provides: {"PlaceCollector"}
// Dependencies: {}
struct PlaceCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , map : & 'a mut Map < 'tcx > , assignments : FxIndexSet < (PlaceIndex , PlaceIndex) > , }
};
}
