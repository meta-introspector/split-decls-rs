macro_rules! deps {
    () => {
        B1!();
        TrimTrailingZeros!();
        InvertedUnsigned!();
        InvertedUInt!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned > TrimTrailingZeros for InvertedUInt < IU , B1 > { type Output = Self ; # [inline] fn trim_trailing_zeros (self) -> Self :: Output { self } }
    };
}

impl_250!();