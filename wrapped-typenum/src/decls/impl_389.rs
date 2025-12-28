macro_rules! deps {
    () => {
        B0!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> | UInt<Ur, B0> = UInt<Ul | Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > BitOr < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : BitOr < Ur > , { type Output = UInt < < Ul as BitOr < Ur > > :: Output , B0 > ; # [inline] fn bitor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . bitor (rhs . msb) , lsb : B0 , } } }
    };
}

impl_389!()