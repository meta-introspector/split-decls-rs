macro_rules! deps {
    () => {
        FoldAdd!();
    };
}

macro_rules! FoldSum {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `FoldAdd`: `FoldSum<A> = <A as FoldAdd>::Output`"] pub type FoldSum < A > = < A as FoldAdd > :: Output ;
    };
}

FoldSum!();