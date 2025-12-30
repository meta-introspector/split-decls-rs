// Generated macro for excluded_locals (function)
macro_rules! Depcrate_value_analysisexcluded_locals {
() => {
// Module: crate::value_analysis
// Provides: {"excluded_locals"}
// Dependencies: {}
# [doc = " Returns all locals with projections that have their reference or address taken."] pub fn excluded_locals (body : & Body < '_ >) -> DenseBitSet < Local > { struct Collector { result : DenseBitSet < Local > , } impl < 'tcx > Visitor < 'tcx > for Collector { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , _location : Location) { if context . may_observe_address () && ! place . is_indirect () { self . result . insert (place . local) ; } } } let mut collector = Collector { result : DenseBitSet :: new_empty (body . local_decls . len ()) } ; collector . visit_body (body) ; collector . result }
};
}
