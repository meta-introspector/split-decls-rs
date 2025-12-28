macro_rules! Shleft {
    () => {
        # [doc = " Alias for the associated type of `Shl`: `Shleft<A, B> = <A as Shl<B>>::Output`"] pub type Shleft < A , B > = < A as Shl < B > > :: Output ;
    };
}

Shleft!()