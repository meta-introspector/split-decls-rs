macro_rules! Diff {
    () => {
        # [doc = " Alias for the associated type of `Sub`: `Diff<A, B> = <A as Sub<B>>::Output`"] pub type Diff < A , B > = < A as Sub < B > > :: Output ;
    };
}

Diff!()