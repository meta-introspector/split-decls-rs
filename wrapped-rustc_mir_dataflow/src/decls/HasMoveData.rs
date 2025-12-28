macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! HasMoveData {
    () => {
        deps!();
        pub trait HasMoveData < 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > ; }
    };
}

HasMoveData!();