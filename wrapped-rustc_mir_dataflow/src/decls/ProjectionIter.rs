macro_rules! deps {
    () => {
        SlicePlusOne!();
    };
}

macro_rules! ProjectionIter {
    () => {
        deps!();
        # [doc = " The iterator returned by [`UnDerefer::iter_projections`]."] struct ProjectionIter < 'a , 'tcx > { places : SlicePlusOne < 'a , PlaceRef < 'tcx > > , proj_idx : usize , }
    };
}

ProjectionIter!();