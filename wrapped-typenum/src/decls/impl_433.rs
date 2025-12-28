macro_rules! deps {
    () => {
        Ord!();
        PrivateCmpOut!();
        Unsigned!();
        UInt!();
        B0!();
        PrivateCmp!();
        B1!();
        Greater!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        # [doc = " Comparing non-terimal bits, with `Lhs` having bit `B1` and `Rhs` having bit `B0`."] # [doc = " `SoFar`, Lhs is `Greater`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B0 > , SoFar > for UInt < Ul , B1 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , Greater > , { type Output = PrivateCmpOut < Ul , Ur , Greater > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B0 > , _ : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Greater) } }
    };
}

impl_433!();