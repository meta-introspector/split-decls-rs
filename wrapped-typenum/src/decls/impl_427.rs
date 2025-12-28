macro_rules! deps {
    () => {
        InternalMarker!();
        Unsigned!();
        B1!();
        Equal!();
        PrivateCmp!();
        UInt!();
        PrivateCmpOut!();
        Cmp!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1>` cmp with `UInt<Ur, B1>`: `SoFar` is `Equal`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateCmp < Ur , Equal > , { type Output = PrivateCmpOut < Ul , Ur , Equal > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B1 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Equal) } }
    };
}

impl_427!();