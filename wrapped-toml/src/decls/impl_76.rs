macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Sealed for str { }
    };
}

impl_76!()