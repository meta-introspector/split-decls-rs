macro_rules! deps {
    () => {
        Ord!();
        B0!();
        UInt!();
        PrivateCmp!();
        Equal!();
        Unsigned!();
        PrivateCmpOut!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        # [doc = " Comparing non-terimal bits, with both having bit `B0`."] # [doc = " These are `Equal`, so we propagate `SoFar`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B0 > , SoFar > for UInt < Ul , B0 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , SoFar > , { type Output = PrivateCmpOut < Ul , Ur , SoFar > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B0 > , so_far : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , so_far) } }
    };
}

impl_430!();