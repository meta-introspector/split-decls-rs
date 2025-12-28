macro_rules! deps {
    () => {
        Trim!();
    };
}

macro_rules! TrimOut {
    () => {
        deps!();
        pub type TrimOut < A > = < A as Trim > :: Output ;
    };
}

TrimOut!()