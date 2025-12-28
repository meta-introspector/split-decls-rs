macro_rules! deps {
    () => {
        BitDiff!();
    };
}

macro_rules! BitDiffOut {
    () => {
        deps!();
        pub type BitDiffOut < A , Rhs > = < A as BitDiff < Rhs > > :: Output ;
    };
}

BitDiffOut!()