macro_rules! deps {
    () => {
        B1!();
        Greater!();
        IsNotEqualPrivate!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < A , B > IsNotEqualPrivate < B , Greater > for A { type Output = True ; # [inline] fn is_not_equal_private (self , _ : B , _ : Greater) -> Self :: Output { B1 } }
    };
}

impl_290!();