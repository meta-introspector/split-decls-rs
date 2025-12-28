macro_rules! Shright {
    () => {
        # [doc = " Alias for the associated type of `Shr`: `Shright<A, B> = <A as Shr<B>>::Output`"] pub type Shright < A , B > = < A as Shr < B > > :: Output ;
    };
}

Shright!();