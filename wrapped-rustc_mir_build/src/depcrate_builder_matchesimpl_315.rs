// Generated macro for impl_315 (impl)
macro_rules! Depcrate_builder_matchesimpl_315 {
() => {
// Module: crate::builder::matches
// Provides: {"impl_315"}
// Dependencies: {}
impl < 'tcx > FlatPat < 'tcx > { # [doc = " Creates a `FlatPat` containing a simplified [`MatchPairTree`] list/forest"] # [doc = " for the given pattern."] fn new (place : PlaceBuilder < 'tcx > , pattern : & Pat < 'tcx > , cx : & mut Builder < '_ , 'tcx >) -> Self { let mut match_pairs = vec ! [] ; let mut extra_data = PatternExtraData { span : pattern . span , bindings : Vec :: new () , ascriptions : Vec :: new () , is_never : pattern . is_never_pattern () , } ; MatchPairTree :: for_pattern (place , pattern , cx , & mut match_pairs , & mut extra_data) ; Self { match_pairs , extra_data } } }
};
}
