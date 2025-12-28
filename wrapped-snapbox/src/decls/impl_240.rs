macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl Eq for RedactedValueInner { }
    };
}

impl_240!()