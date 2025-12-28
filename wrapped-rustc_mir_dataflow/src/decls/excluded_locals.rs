macro_rules! excluded_locals {
    () => {
        # [doc = " Returns all locals with projections that have their reference or address taken."] pub fn excluded_locals (body : & Body < '_ >) -> DenseBitSet < Local > { struct Collector { result : DenseBitSet < Local > , } impl < 'tcx > Visitor < 'tcx > for Collector { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , _location : Location) { if context . may_observe_address () && ! place . is_indirect () { self . result . insert (place . local) ; } } } let mut collector = Collector { result : DenseBitSet :: new_empty (body . local_decls . len ()) } ; collector . visit_body (body) ; collector . result }
    };
}

excluded_locals!();