macro_rules! deps {
    () => {
        GetBit!();
        UInt!();
        Internal!();
        Sub1!();
        GetBitOut!();
        B1!();
        InternalMarker!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl < Un , Bn , Ui , Bi > GetBit < UInt < Ui , Bi > > for UInt < Un , Bn > where UInt < Ui , Bi > : Copy + Sub < B1 > , Un : GetBit < Sub1 < UInt < Ui , Bi > > > , { type Output = GetBitOut < Un , Sub1 < UInt < Ui , Bi > > > ; # [inline] fn get_bit < IM : InternalMarker > (& self , i : & UInt < Ui , Bi >) -> Self :: Output { self . msb . get_bit :: < Internal > (& (* i - B1)) } }
    };
}

impl_458!();