macro_rules! deps {
    () => {
        Equal!();
        InternalMarker!();
        B0!();
        Cmp!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Cmp < B0 > for B0 { type Output = Equal ; # [inline] fn compare < P : InternalMarker > (& self , _ : & B0) -> Self :: Output { Equal } }
    };
}

impl_22!()