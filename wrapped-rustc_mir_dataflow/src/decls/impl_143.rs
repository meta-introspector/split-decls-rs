macro_rules! deps {
    () => {
        HasMoveData!();
        EverInitializedPlaces!();
        MoveData!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'tcx > HasMoveData < 'tcx > for EverInitializedPlaces < '_ , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }
    };
}

impl_143!();