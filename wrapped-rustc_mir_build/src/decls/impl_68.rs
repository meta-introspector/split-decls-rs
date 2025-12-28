macro_rules! deps {
    () => {
        PlaceBase!();
        PlaceBuilder!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'tcx > From < PlaceBase > for PlaceBuilder < 'tcx > { fn from (base : PlaceBase) -> Self { Self { base , projection : Vec :: new () } } }
    };
}

impl_68!()