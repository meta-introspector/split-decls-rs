macro_rules! deps {
    () => {
        InternalMarker!();
        PInt!();
        NInt!();
        Less!();
        Unsigned!();
        Cmp!();
        NonZero!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        # [doc = " -X < Y"] impl < P : Unsigned + NonZero , N : Unsigned + NonZero > Cmp < PInt < P > > for NInt < N > { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & PInt < P >) -> Self :: Output { Less } }
    };
}

impl_97!();