macro_rules! deps {
    () => {
        MoveData!();
        HasMoveData!();
        MaybeUninitializedPlaces!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'tcx > HasMoveData < 'tcx > for MaybeUninitializedPlaces < '_ , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }
    };
}

impl_140!()