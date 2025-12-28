macro_rules! deps {
    () => {
        ByteSliceMut!();
        SplitByteSlice!();
        SplitByteSliceMut!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < B : SplitByteSlice + ByteSliceMut > SplitByteSliceMut for B { }
    };
}

impl_98!();