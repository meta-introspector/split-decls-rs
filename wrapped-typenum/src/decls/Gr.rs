macro_rules! deps {
    () => {
        IsGreater!();
    };
}

macro_rules! Gr {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsGreater`: `Gr<A, B> = <A as IsGreater<B>>::Output`"] pub type Gr < A , B > = < A as IsGreater < B > > :: Output ;
    };
}

Gr!();