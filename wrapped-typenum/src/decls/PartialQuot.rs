macro_rules! deps {
    () => {
        PartialDiv!();
    };
}

macro_rules! PartialQuot {
    () => {
        deps!();
        # [doc = " Alias for the associated type of"] # [doc = " `PartialDiv`: `PartialQuot<A, B> = <A as PartialDiv<B>>::Output`"] pub type PartialQuot < A , B > = < A as PartialDiv < B > > :: Output ;
    };
}

PartialQuot!();