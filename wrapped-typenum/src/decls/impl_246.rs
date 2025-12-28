macro_rules! deps {
    () => {
        InvertedUTerm!();
        Unsigned!();
        PrivateInvert!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < U : Unsigned > PrivateInvert < U > for InvertedUTerm { type Output = U ; # [inline] fn private_invert (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_246!()