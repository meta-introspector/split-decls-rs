macro_rules! deps {
    () => {
        TrimOut!();
        Bit!();
        Unsigned!();
        PrivateXorOut!();
        PrivateXor!();
        Trim!();
        UInt!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        # [doc = " Xoring unsigned integers."] # [doc = " We use our `PrivateXor` operator and then `Trim` the output."] impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned > BitXor < Ur > for UInt < Ul , Bl > where UInt < Ul , Bl > : PrivateXor < Ur > , PrivateXorOut < UInt < Ul , Bl > , Ur > : Trim , { type Output = TrimOut < PrivateXorOut < UInt < Ul , Bl > , Ur > > ; # [inline] fn bitxor (self , rhs : Ur) -> Self :: Output { self . private_xor (rhs) . trim () } }
    };
}

impl_394!();