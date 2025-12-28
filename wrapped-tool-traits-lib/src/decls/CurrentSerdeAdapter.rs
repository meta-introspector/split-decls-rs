macro_rules! deps {
    () => {
        RealSerdeAdapter!();
    };
}

macro_rules! CurrentSerdeAdapter {
    () => {
        deps!();
        pub type CurrentSerdeAdapter = RealSerdeAdapter ;
    };
}

CurrentSerdeAdapter!();