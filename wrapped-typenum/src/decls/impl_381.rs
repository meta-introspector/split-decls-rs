macro_rules! deps {
    () => {
        UTerm!();
        PrivateAnd!();
        Unsigned!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        # [doc = " `UTerm & X = UTerm`"] impl < U : Unsigned > PrivateAnd < U > for UTerm { type Output = UTerm ; # [inline] fn private_and (self , _ : U) -> Self :: Output { UTerm } }
    };
}

impl_381!();