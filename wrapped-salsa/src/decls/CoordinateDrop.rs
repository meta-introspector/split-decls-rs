macro_rules! deps {
    () => {
        Coordinate!();
    };
}

macro_rules! CoordinateDrop {
    () => {
        deps!();
        struct CoordinateDrop (Arc < Coordinate >) ;
    };
}

CoordinateDrop!();