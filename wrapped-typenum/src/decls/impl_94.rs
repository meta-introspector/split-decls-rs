macro_rules! deps {
    () => {
        Z0!();
        Less!();
        Unsigned!();
        PInt!();
        InternalMarker!();
        Cmp!();
        NonZero!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        # [doc = " 0 < X"] impl < U : Unsigned + NonZero > Cmp < PInt < U > > for Z0 { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & PInt < U >) -> Self :: Output { Less } }
    };
}

impl_94!()