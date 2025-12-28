macro_rules! deps {
    () => {
        Prod!();
        PrivatePow!();
        Unsigned!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl < Y : Unsigned , X : Unsigned > PrivatePow < Y , U1 > for X where X : Mul < Y > , { type Output = Prod < X , Y > ; # [inline] fn private_pow (self , y : Y , _ : U1) -> Self :: Output { self * y } }
    };
}

impl_442!();