macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T : Value > crate :: sealed :: Sealed for Option < T > { }
    };
}

impl_165!();