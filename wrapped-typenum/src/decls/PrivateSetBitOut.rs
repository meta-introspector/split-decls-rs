macro_rules! deps {
    () => {
        PrivateSetBit!();
    };
}

macro_rules! PrivateSetBitOut {
    () => {
        deps!();
        pub type PrivateSetBitOut < N , I , B > = < N as PrivateSetBit < I , B > > :: Output ;
    };
}

PrivateSetBitOut!();