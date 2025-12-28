macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        PrivateXor!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [doc = " `UTerm ^ X = X`"] impl < U : Unsigned > PrivateXor < U > for UTerm { type Output = U ; # [inline] fn private_xor (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_395!();