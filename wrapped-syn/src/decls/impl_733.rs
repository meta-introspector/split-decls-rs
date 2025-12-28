macro_rules! deps {
    () => {
        ThreadBound!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl < T : Copy > Copy for ThreadBound < T > { }
    };
}

impl_733!()