macro_rules! deps {
    () => {
        Less!();
        Cmp!();
        B1!();
        B0!();
        InternalMarker!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Cmp < B1 > for B0 { type Output = Less ; # [inline] fn compare < P : InternalMarker > (& self , _ : & B1) -> Self :: Output { Less } }
    };
}

impl_23!()