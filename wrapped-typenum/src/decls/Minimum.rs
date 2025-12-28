macro_rules! deps {
    () => {
        Min!();
    };
}

macro_rules! Minimum {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Min`: `Minimum<A, B> = <A as Min<B>>::Output`"] pub type Minimum < A , B > = < A as Min < B > > :: Output ;
    };
}

Minimum!()