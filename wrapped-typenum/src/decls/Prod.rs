macro_rules! Prod {
    () => {
        # [doc = " Alias for the associated type of `Mul`: `Prod<A, B> = <A as Mul<B>>::Output`"] pub type Prod < A , B > = < A as Mul < B > > :: Output ;
    };
}

Prod!();