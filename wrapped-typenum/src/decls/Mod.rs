macro_rules! Mod {
    () => {
        # [doc = " Alias for the associated type of `Rem`: `Mod<A, B> = <A as Rem<B>>::Output`"] pub type Mod < A , B > = < A as Rem < B > > :: Output ;
    };
}

Mod!()