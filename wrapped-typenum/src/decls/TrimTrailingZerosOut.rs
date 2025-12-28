macro_rules! deps {
    () => {
        TrimTrailingZeros!();
    };
}

macro_rules! TrimTrailingZerosOut {
    () => {
        deps!();
        pub type TrimTrailingZerosOut < A > = < A as TrimTrailingZeros > :: Output ;
    };
}

TrimTrailingZerosOut!();