macro_rules! deps {
    () => {
        ShiftDiff!();
    };
}

macro_rules! ShiftDiffOut {
    () => {
        deps!();
        pub type ShiftDiffOut < A , Rhs > = < A as ShiftDiff < Rhs > > :: Output ;
    };
}

ShiftDiffOut!()