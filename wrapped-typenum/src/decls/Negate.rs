macro_rules! Negate {
    () => {
        # [doc = " Alias for the associated type of `Neg`: `Negate<A> = <A as Neg>::Output`"] pub type Negate < A > = < A as Neg > :: Output ;
    };
}

Negate!()