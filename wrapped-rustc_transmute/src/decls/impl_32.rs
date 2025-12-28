macro_rules! deps {
    () => {
        Region!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Region for ! { }
    };
}

impl_32!()