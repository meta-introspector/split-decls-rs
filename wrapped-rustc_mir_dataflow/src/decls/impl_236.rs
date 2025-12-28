macro_rules! deps {
    () => {
        SlicePlusOne!();
        ProjectionIter!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < 'a , 'tcx > ProjectionIter < 'a , 'tcx > { # [inline] fn new (deref_chain : & 'a [PlaceRef < 'tcx >] , place : PlaceRef < 'tcx >) -> Self { let last = if place . as_local () . is_none () { Some (place) } else { debug_assert ! (deref_chain . is_empty ()) ; None } ; ProjectionIter { places : SlicePlusOne { slice : deref_chain , last } , proj_idx : 0 } } }
    };
}

impl_236!();