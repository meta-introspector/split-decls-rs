macro_rules! deps {
    () => {
        PrivatePow!();
        Unsigned!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl < Y : Unsigned , X : Unsigned > PrivatePow < Y , U0 > for X { type Output = Y ; # [inline] fn private_pow (self , y : Y , _ : U0) -> Self :: Output { y } }
    };
}

impl_441!()