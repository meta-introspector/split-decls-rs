macro_rules! deps {
    () => {
        SquareRoot!();
    };
}

macro_rules! Sqrt {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `SquareRoot`: `Sqrt<A> = <A as SquareRoot>::Output`"] pub type Sqrt < A > = < A as SquareRoot > :: Output ;
    };
}

Sqrt!();