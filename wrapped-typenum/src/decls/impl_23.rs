macro_rules! deps {
    () => {
        B1!();
        Cmp!();
        InternalMarker!();
        B0!();
        Less!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Cmp < B1 > for B0 { type Output = Less ; # [inline] fn compare < P : InternalMarker > (& self , _ : & B1) -> Self :: Output { Less } }
    };
}

impl_23!();