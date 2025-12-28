macro_rules! deps {
    () => {
        Cmp!();
        NInt!();
        Greater!();
        Z0!();
        InternalMarker!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [doc = " 0 > -X"] impl < U : Unsigned + NonZero > Cmp < NInt < U > > for Z0 { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & NInt < U >) -> Self :: Output { Greater } }
    };
}

impl_93!()