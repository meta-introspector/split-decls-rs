macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T : ? Sized > crate :: sealed :: Sealed for Box < T > where T : Value { }
    };
}

impl_146!()