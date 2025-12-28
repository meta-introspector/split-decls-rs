macro_rules! deps {
    () => {
        IsLess!();
    };
}

macro_rules! Le {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsLess`: `Le<A, B> = <A as IsLess<B>>::Output`"] pub type Le < A , B > = < A as IsLess < B > > :: Output ;
    };
}

Le!();