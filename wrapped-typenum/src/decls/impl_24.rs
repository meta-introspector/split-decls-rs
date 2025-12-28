macro_rules! deps {
    () => {
        Cmp!();
        B0!();
        Greater!();
        B1!();
        InternalMarker!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Cmp < B0 > for B1 { type Output = Greater ; # [inline] fn compare < P : InternalMarker > (& self , _ : & B0) -> Self :: Output { Greater } }
    };
}

impl_24!();