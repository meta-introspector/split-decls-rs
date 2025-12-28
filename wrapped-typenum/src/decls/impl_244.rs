macro_rules! deps {
    () => {
        InvertedUTerm!();
        UTerm!();
        Invert!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl Invert for InvertedUTerm { type Output = UTerm ; # [inline] fn invert (self) -> Self :: Output { UTerm } }
    };
}

impl_244!();