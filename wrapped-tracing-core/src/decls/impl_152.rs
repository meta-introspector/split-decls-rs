macro_rules! deps {
    () => {
        DisplayValue!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < T : fmt :: Display > crate :: sealed :: Sealed for DisplayValue < T > { }
    };
}

impl_152!();