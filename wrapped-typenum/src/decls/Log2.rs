macro_rules! deps {
    () => {
        Logarithm2!();
    };
}

macro_rules! Log2 {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Logarithm2`: `Log2<A> = <A as Logarithm2>::Output`"] pub type Log2 < A > = < A as Logarithm2 > :: Output ;
    };
}

Log2!()