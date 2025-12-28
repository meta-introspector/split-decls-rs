macro_rules! deps {
    () => {
        Max!();
    };
}

macro_rules! Maximum {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Max`: `Maximum<A, B> = <A as Max<B>>::Output`"] pub type Maximum < A , B > = < A as Max < B > > :: Output ;
    };
}

Maximum!()