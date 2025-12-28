macro_rules! deps {
    () => {
        Square!();
    };
}

macro_rules! Cube {
    () => {
        deps!();
        # [doc = " Alias to make it easy to cube. `Cube<A> = <Square<A> as Mul<A>>::Output`"] pub type Cube < A > = < Square < A > as Mul < A > > :: Output ;
    };
}

Cube!()