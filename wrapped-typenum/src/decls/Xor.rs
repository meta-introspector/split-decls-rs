macro_rules! Xor {
    () => {
        # [doc = " Alias for the associated type of `BitXor`: `Xor<A, B> = <A as BitXor<B>>::Output`"] pub type Xor < A , B > = < A as BitXor < B > > :: Output ;
    };
}

Xor!()