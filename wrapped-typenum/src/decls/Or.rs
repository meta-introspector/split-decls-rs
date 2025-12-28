macro_rules! Or {
    () => {
        # [doc = " Alias for the associated type of `BitOr`: `Or<A, B> = <A as BitOr<B>>::Output`"] pub type Or < A , B > = < A as BitOr < B > > :: Output ;
    };
}

Or!()