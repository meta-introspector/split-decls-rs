macro_rules! deps {
    () => {
        B1!();
        InternalMarker!();
        Less!();
        Unsigned!();
        B0!();
        PrivateCmp!();
        PrivateCmpOut!();
        UInt!();
        Cmp!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0>` cmp with `UInt<Ur, B1>`: `SoFar` is `Less`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : PrivateCmp < Ur , Less > , { type Output = PrivateCmpOut < Ul , Ur , Less > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B1 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Less) } }
    };
}

impl_428!()