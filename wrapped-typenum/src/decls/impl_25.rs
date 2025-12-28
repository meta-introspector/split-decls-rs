macro_rules! deps {
    () => {
        Cmp!();
        B1!();
        Equal!();
        InternalMarker!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Cmp < B1 > for B1 { type Output = Equal ; # [inline] fn compare < P : InternalMarker > (& self , _ : & B1) -> Self :: Output { Equal } }
    };
}

impl_25!();