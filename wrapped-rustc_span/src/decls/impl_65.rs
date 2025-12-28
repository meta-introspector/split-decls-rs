macro_rules! deps {
    () => {
        ExpnData!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl ! PartialEq for ExpnData { }
    };
}

impl_65!()