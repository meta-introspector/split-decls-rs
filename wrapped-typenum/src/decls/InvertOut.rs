macro_rules! deps {
    () => {
        Invert!();
    };
}

macro_rules! InvertOut {
    () => {
        deps!();
        pub type InvertOut < A > = < A as Invert > :: Output ;
    };
}

InvertOut!()