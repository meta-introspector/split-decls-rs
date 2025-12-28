macro_rules! deps {
    () => {
        Less!();
        Unsigned!();
        Zero!();
        Bit!();
        UTerm!();
        Cmp!();
        UInt!();
        InternalMarker!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        # [doc = " Zero < Nonzero"] impl < U : Unsigned , B : Bit > Cmp < UInt < U , B > > for UTerm { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & UInt < U , B >) -> Self :: Output { Less } }
    };
}

impl_425!()