macro_rules! deps {
    () => {
        ExpnData!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ! Hash for ExpnData { }
    };
}

impl_66!()