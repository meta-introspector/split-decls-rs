macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! INCOMING {
    () => {
        deps!();
        pub const INCOMING : Direction = Direction { repr : 1 } ;
    };
}

INCOMING!();