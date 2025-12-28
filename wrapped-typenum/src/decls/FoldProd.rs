macro_rules! deps {
    () => {
        FoldMul!();
    };
}

macro_rules! FoldProd {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `FoldMul`: `FoldProd<A> = <A as FoldMul>::Output`"] pub type FoldProd < A > = < A as FoldMul > :: Output ;
    };
}

FoldProd!()