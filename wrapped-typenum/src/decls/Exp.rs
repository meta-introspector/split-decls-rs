macro_rules! deps {
    () => {
        Pow!();
    };
}

macro_rules! Exp {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Pow`: `Exp<A, B> = <A as Pow<B>>::Output`"] pub type Exp < A , B > = < A as Pow < B > > :: Output ;
    };
}

Exp!();