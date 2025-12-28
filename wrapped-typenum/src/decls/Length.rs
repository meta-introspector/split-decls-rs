macro_rules! deps {
    () => {
        Len!();
    };
}

macro_rules! Length {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Len`: `Length<A> = <A as Len>::Output`"] pub type Length < T > = < T as Len > :: Output ;
    };
}

Length!()