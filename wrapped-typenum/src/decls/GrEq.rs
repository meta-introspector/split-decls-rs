macro_rules! deps {
    () => {
        IsGreaterOrEqual!();
    };
}

macro_rules! GrEq {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsGreaterOrEqual`:"] # [doc = " `GrEq<A, B> = <A as IsGreaterOrEqual<B>>::Output`"] pub type GrEq < A , B > = < A as IsGreaterOrEqual < B > > :: Output ;
    };
}

GrEq!()