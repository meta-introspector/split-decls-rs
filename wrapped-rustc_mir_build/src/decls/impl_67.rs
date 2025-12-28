macro_rules! deps {
    () => {
        PlaceBuilder!();
        PlaceBase!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'tcx > From < Local > for PlaceBuilder < 'tcx > { fn from (local : Local) -> Self { Self { base : PlaceBase :: Local (local) , projection : Vec :: new () } } }
    };
}

impl_67!();