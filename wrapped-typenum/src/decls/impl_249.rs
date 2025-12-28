macro_rules! deps {
    () => {
        InvertedUTerm!();
        TrimTrailingZeros!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl TrimTrailingZeros for InvertedUTerm { type Output = InvertedUTerm ; # [inline] fn trim_trailing_zeros (self) -> Self :: Output { InvertedUTerm } }
    };
}

impl_249!()