macro_rules! deps {
    () => {
        NonZero!();
        PInt!();
        InternalMarker!();
        Greater!();
        Unsigned!();
        Cmp!();
        NInt!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        # [doc = " X > - Y"] impl < P : Unsigned + NonZero , N : Unsigned + NonZero > Cmp < NInt < N > > for PInt < P > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & NInt < N >) -> Self :: Output { Greater } }
    };
}

impl_98!();