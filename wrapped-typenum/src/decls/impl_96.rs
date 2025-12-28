macro_rules! deps {
    () => {
        NonZero!();
        InternalMarker!();
        Unsigned!();
        Cmp!();
        Z0!();
        Less!();
        NInt!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [doc = " -X < 0"] impl < U : Unsigned + NonZero > Cmp < Z0 > for NInt < U > { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & Z0) -> Self :: Output { Less } }
    };
}

impl_96!()