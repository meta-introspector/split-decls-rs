macro_rules! deps {
    () => {
        BlockedOnInner!();
    };
}

macro_rules! Running {
    () => {
        deps!();
        pub struct Running < 'me > (Box < BlockedOnInner < 'me > >) ;
    };
}

Running!()