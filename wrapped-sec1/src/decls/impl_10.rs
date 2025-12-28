macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < Size : ModulusSize > Eq for EncodedPoint < Size > { }
    };
}

impl_10!();