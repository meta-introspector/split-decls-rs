macro_rules! Quot {
    () => {
        # [doc = " Alias for the associated type of `Div`: `Quot<A, B> = <A as Div<B>>::Output`"] pub type Quot < A , B > = < A as Div < B > > :: Output ;
    };
}

Quot!();