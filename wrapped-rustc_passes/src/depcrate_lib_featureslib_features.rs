// Generated macro for lib_features (function)
macro_rules! Depcrate_lib_featureslib_features {
() => {
// Module: crate::lib_features
// Provides: {"lib_features"}
// Dependencies: {}
fn lib_features (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> LibFeatures { if ! tcx . features () . staged_api () { return LibFeatures :: default () ; } let mut collector = LibFeatureCollector :: new (tcx) ; tcx . hir_walk_attributes (& mut collector) ; collector . lib_features }
};
}
