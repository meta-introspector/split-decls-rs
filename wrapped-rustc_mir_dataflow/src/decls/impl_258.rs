macro_rules! deps {
    () => {
        PlaceInfo!();
        TrackElem!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'tcx > PlaceInfo < 'tcx > { fn new (ty : Ty < 'tcx > , proj_elem : Option < TrackElem >) -> Self { Self { ty , next_sibling : None , first_child : None , proj_elem , value_index : None } } }
    };
}

impl_258!()