macro_rules! deps {
    () => {
        SetBit!();
    };
}

macro_rules! SetBitOut {
    () => {
        deps!();
        # [doc = " Alias for the result of calling `SetBit`: `SetBitOut<N, I, B> = <N as SetBit<I, B>>::Output`."] pub type SetBitOut < N , I , B > = < N as SetBit < I , B > > :: Output ;
    };
}

SetBitOut!()