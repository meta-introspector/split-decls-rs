macro_rules! deps {
    () => {
        Square!();
        Bit!();
        PrivatePow!();
        UInt!();
        Unsigned!();
        B1!();
        Prod!();
        PrivatePowOut!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        # [doc = " N is odd"] impl < Y : Unsigned , U : Unsigned , B : Bit , X : Unsigned > PrivatePow < Y , UInt < UInt < U , B > , B1 > > for X where X : Mul + Mul < Y > , Square < X > : PrivatePow < Prod < X , Y > , UInt < U , B > > , { type Output = PrivatePowOut < Square < X > , Prod < X , Y > , UInt < U , B > > ; # [inline] fn private_pow (self , y : Y , n : UInt < UInt < U , B > , B1 >) -> Self :: Output { (self * self) . private_pow (self * y , n . msb) } }
    };
}

impl_444!();