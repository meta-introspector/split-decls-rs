macro_rules! deps {
    () => {
        Zero!();
        Cmp!();
        Unsigned!();
        UTerm!();
        UInt!();
        Greater!();
        Bit!();
        InternalMarker!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [doc = " Nonzero > Zero"] impl < U : Unsigned , B : Bit > Cmp < UTerm > for UInt < U , B > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & UTerm) -> Self :: Output { Greater } }
    };
}

impl_424!();