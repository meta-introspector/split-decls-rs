macro_rules! deps {
    () => {
        PrivatePow!();
        UInt!();
        Unsigned!();
        PrivatePowOut!();
        Bit!();
        Square!();
        B0!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        # [doc = " N is even"] impl < Y : Unsigned , U : Unsigned , B : Bit , X : Unsigned > PrivatePow < Y , UInt < UInt < U , B > , B0 > > for X where X : Mul , Square < X > : PrivatePow < Y , UInt < U , B > > , { type Output = PrivatePowOut < Square < X > , Y , UInt < U , B > > ; # [inline] fn private_pow (self , y : Y , n : UInt < UInt < U , B > , B0 >) -> Self :: Output { (self * self) . private_pow (y , n . msb) } }
    };
}

impl_443!();