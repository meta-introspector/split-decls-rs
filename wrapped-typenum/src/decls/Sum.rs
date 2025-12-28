macro_rules! Sum {
    () => {
        # [doc = " Alias for the associated type of `Add`: `Sum<A, B> = <A as Add<B>>::Output`"] pub type Sum < A , B > = < A as Add < B > > :: Output ;
    };
}

Sum!();