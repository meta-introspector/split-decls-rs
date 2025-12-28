macro_rules! deps {
    () => {
        Abs!();
    };
}

macro_rules! AbsVal {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Abs`: `AbsVal<A> = <A as Abs>::Output`"] pub type AbsVal < A > = < A as Abs > :: Output ;
    };
}

AbsVal!()