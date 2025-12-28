macro_rules! deps {
    () => {
        GetBit!();
    };
}

macro_rules! GetBitOut {
    () => {
        deps!();
        # [allow (missing_docs)] pub type GetBitOut < N , I > = < N as GetBit < I > > :: Output ;
    };
}

GetBitOut!();