macro_rules! Square {
    () => {
        # [doc = " Alias to make it easy to square. `Square<A> = <A as Mul<A>>::Output`"] pub type Square < A > = < A as Mul > :: Output ;
    };
}

Square!()