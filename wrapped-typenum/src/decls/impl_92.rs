macro_rules! deps {
    () => {
        InternalMarker!();
        Z0!();
        Cmp!();
        Equal!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [doc = " 0 == 0"] impl Cmp < Z0 > for Z0 { type Output = Equal ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & Z0) -> Self :: Output { Equal } }
    };
}

impl_92!()