macro_rules! deps {
    () => {
        InternalMarker!();
        Cmp!();
        Equal!();
        UTerm!();
        Zero!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        # [doc = " Zero == Zero"] impl Cmp < UTerm > for UTerm { type Output = Equal ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & UTerm) -> Self :: Output { Equal } }
    };
}

impl_423!();