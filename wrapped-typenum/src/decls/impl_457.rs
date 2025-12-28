macro_rules! deps {
    () => {
        GetBit!();
        UInt!();
        InternalMarker!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl < Un , Bn > GetBit < U0 > for UInt < Un , Bn > where Bn : Copy , { type Output = Bn ; # [inline] fn get_bit < IM : InternalMarker > (& self , _ : & U0) -> Self :: Output { self . lsb } }
    };
}

impl_457!()