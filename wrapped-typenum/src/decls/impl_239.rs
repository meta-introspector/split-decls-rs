macro_rules! deps {
    () => {
        InvertedUTerm!();
        Invert!();
        UTerm!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl Invert for UTerm { type Output = InvertedUTerm ; # [inline] fn invert (self) -> Self :: Output { InvertedUTerm } }
    };
}

impl_239!();