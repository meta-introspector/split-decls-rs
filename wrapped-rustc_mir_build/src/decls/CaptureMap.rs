macro_rules! deps {
    () => {
        Capture!();
    };
}

macro_rules! CaptureMap {
    () => {
        deps!();
        type CaptureMap < 'tcx > = SortedIndexMultiMap < usize , ItemLocalId , Capture < 'tcx > > ;
    };
}

CaptureMap!()