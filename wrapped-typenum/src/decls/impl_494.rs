macro_rules! deps {
    () => {
        PrivateSquareRoot!();
        SquareRoot!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < N > SquareRoot for N where N : PrivateSquareRoot , { type Output = < Self as PrivateSquareRoot > :: Output ; }
    };
}

impl_494!();