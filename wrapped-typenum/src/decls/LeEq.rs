macro_rules! deps {
    () => {
        IsLessOrEqual!();
    };
}

macro_rules! LeEq {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `IsLessOrEqual`: `LeEq<A, B> = <A as IsLessOrEqual<B>>::Output`"] pub type LeEq < A , B > = < A as IsLessOrEqual < B > > :: Output ;
    };
}

LeEq!();