macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl crate :: sealed :: Sealed for Empty { }
    };
}

impl_163!()