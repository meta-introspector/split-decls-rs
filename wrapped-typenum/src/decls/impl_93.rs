macro_rules! deps {
    () => {
        NonZero!();
        Z0!();
        InternalMarker!();
        Cmp!();
        Unsigned!();
        Greater!();
        NInt!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [doc = " 0 > -X"] impl < U : Unsigned + NonZero > Cmp < NInt < U > > for Z0 { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & NInt < U >) -> Self :: Output { Greater } }
    };
}

impl_93!();