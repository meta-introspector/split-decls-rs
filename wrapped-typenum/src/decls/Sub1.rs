macro_rules! deps {
    () => {
        B1!();
    };
}

macro_rules! Sub1 {
    () => {
        deps!();
        # [doc = " Alias to make it easy to subtract 1: `Sub1<A> = <A as Sub<B1>>::Output`"] pub type Sub1 < A > = < A as Sub < crate :: bit :: B1 > > :: Output ;
    };
}

Sub1!()