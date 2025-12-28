macro_rules! deps {
    () => {
        TrimTrailingZeros!();
        Unsigned!();
        Trim!();
        Invert!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < U : Unsigned > Trim for U where U : Invert , < U as Invert > :: Output : TrimTrailingZeros , < < U as Invert > :: Output as TrimTrailingZeros > :: Output : Invert , { type Output = < < < U as Invert > :: Output as TrimTrailingZeros > :: Output as Invert > :: Output ; # [inline] fn trim (self) -> Self :: Output { self . invert () . trim_trailing_zeros () . invert () } }
    };
}

impl_252!()