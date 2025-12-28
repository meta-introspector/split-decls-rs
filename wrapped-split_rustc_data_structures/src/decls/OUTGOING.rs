macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! OUTGOING {
    () => {
        deps!();
        pub const OUTGOING : Direction = Direction { repr : 0 } ;
    };
}

OUTGOING!();