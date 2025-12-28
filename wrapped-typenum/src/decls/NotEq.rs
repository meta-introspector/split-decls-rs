macro_rules! deps {
    () => {
        IsNotEqual!();
    };
}

macro_rules! NotEq {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsNotEqual`: `NotEq<A, B> = <A as IsNotEqual<B>>::Output`"] pub type NotEq < A , B > = < A as IsNotEqual < B > > :: Output ;
    };
}

NotEq!()