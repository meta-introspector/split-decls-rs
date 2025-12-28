macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < Size > Copy for EncodedPoint < Size > where Size : ModulusSize , < Size :: UncompressedPointSize as ArraySize > :: ArrayType < u8 > : Copy , { }
    };
}

impl_8!()