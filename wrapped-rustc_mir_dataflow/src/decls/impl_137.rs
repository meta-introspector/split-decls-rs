macro_rules! deps {
    () => {
        MaybeInitializedPlaces!();
        HasMoveData!();
        MoveData!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a , 'tcx > HasMoveData < 'tcx > for MaybeInitializedPlaces < 'a , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }
    };
}

impl_137!();