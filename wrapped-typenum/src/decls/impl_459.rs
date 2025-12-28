macro_rules! deps {
    () => {
        InternalMarker!();
        UTerm!();
        B0!();
        GetBit!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < I > GetBit < I > for UTerm { type Output = B0 ; # [inline] fn get_bit < IM : InternalMarker > (& self , _ : & I) -> Self :: Output { B0 } }
    };
}

impl_459!()