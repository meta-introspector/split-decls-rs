macro_rules! deps {
    () => {
        Unsigned!();
        PrivateCmp!();
        InternalMarker!();
        Cmp!();
        PrivateCmpOut!();
        B0!();
        Equal!();
        UInt!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0>` cmp with `UInt<Ur, B0>`: `SoFar` is `Equal`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateCmp < Ur , Equal > , { type Output = PrivateCmpOut < Ul , Ur , Equal > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B0 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Equal) } }
    };
}

impl_426!();