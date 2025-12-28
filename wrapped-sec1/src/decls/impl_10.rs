macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < Size : ModulusSize > Eq for EncodedPoint < Size > { }
    };
}

impl_10!()