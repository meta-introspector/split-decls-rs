macro_rules! deps {
    () => {
        PrivateCmpOut!();
        PrivateCmp!();
        Unsigned!();
        B1!();
        Equal!();
        UInt!();
        Ord!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        # [doc = " Comparing non-terimal bits, with both having bit `B1`."] # [doc = " These are `Equal`, so we propagate `SoFar`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B1 > , SoFar > for UInt < Ul , B1 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , SoFar > , { type Output = PrivateCmpOut < Ul , Ur , SoFar > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B1 > , so_far : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , so_far) } }
    };
}

impl_431!();