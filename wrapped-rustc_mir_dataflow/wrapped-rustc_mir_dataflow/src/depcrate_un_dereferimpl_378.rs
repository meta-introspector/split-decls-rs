// Generated macro for impl_378 (impl)
macro_rules! Depcrate_un_dereferimpl_378 {
() => {
// Module: crate::un_derefer
// Provides: {"impl_378"}
// Dependencies: {}
impl < 'tcx > Iterator for ProjectionIter < '_ , 'tcx > { type Item = (PlaceRef < 'tcx > , PlaceElem < 'tcx >) ; # [inline] fn next (& mut self) -> Option < (PlaceRef < 'tcx > , PlaceElem < 'tcx >) > { let place = self . places . read () ? ; let partial_place = PlaceRef { local : place . local , projection : & place . projection [.. self . proj_idx] } ; let elem = place . projection [self . proj_idx] ; if self . proj_idx == place . projection . len () - 1 { self . proj_idx = 0 ; self . places . advance () ; } else { self . proj_idx += 1 ; } Some ((partial_place , elem)) } }
};
}
