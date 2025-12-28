macro_rules! deps {
    () => {
        PrivatePowOut!();
        Unsigned!();
        PrivatePow!();
        Pow!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        # [doc = " X^N"] impl < X : Unsigned , N : Unsigned > Pow < N > for X where X : PrivatePow < U1 , N > , { type Output = PrivatePowOut < X , U1 , N > ; # [inline] fn powi (self , n : N) -> Self :: Output { self . private_pow (U1 :: new () , n) } }
    };
}

impl_440!();