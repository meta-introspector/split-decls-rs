macro_rules! deps {
    () => {
        InternalMarker!();
        NonZero!();
        Greater!();
        Unsigned!();
        PInt!();
        Z0!();
        Cmp!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        # [doc = " X > 0"] impl < U : Unsigned + NonZero > Cmp < Z0 > for PInt < U > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & Z0) -> Self :: Output { Greater } }
    };
}

impl_95!();