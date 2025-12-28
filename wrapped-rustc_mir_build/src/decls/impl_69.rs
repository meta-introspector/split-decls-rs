macro_rules! deps {
    () => {
        PlaceBase!();
        PlaceBuilder!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'tcx > From < Place < 'tcx > > for PlaceBuilder < 'tcx > { fn from (p : Place < 'tcx >) -> Self { Self { base : PlaceBase :: Local (p . local) , projection : p . projection . to_vec () } } }
    };
}

impl_69!()