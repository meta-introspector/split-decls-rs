macro_rules! deps {
    () => {
        InvertedUInt!();
        B0!();
        TrimTrailingZeros!();
        InvertedUnsigned!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned > TrimTrailingZeros for InvertedUInt < IU , B0 > where IU : TrimTrailingZeros , { type Output = < IU as TrimTrailingZeros > :: Output ; # [inline] fn trim_trailing_zeros (self) -> Self :: Output { self . msb . trim_trailing_zeros () } }
    };
}

impl_251!();