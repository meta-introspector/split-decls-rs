macro_rules! deps {
    () => {
        Cmp!();
    };
}

macro_rules! Compare {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Cmp`: `Compare<A, B> = <A as Cmp<B>>::Output`"] pub type Compare < A , B > = < A as Cmp < B > > :: Output ;
    };
}

Compare!();