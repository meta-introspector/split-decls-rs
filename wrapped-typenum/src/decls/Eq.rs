macro_rules! deps {
    () => {
        IsEqual!();
    };
}

macro_rules! Eq {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsEqual`: `Eq<A, B> = <A as IsEqual<B>>::Output`"] pub type Eq < A , B > = < A as IsEqual < B > > :: Output ;
    };
}

Eq!();