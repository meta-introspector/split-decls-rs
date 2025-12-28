macro_rules! And {
    () => {
        # [doc = " Alias for the associated type of `BitAnd`: `And<A, B> = <A as BitAnd<B>>::Output`"] pub type And < A , B > = < A as BitAnd < B > > :: Output ;
    };
}

And!();