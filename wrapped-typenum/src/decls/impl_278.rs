macro_rules! deps {
    () => {
        B0!();
        IsEqualPrivate!();
        Greater!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < A , B > IsEqualPrivate < B , Greater > for A { type Output = False ; # [inline] fn is_equal_private (self , _ : B , _ : Greater) -> Self :: Output { B0 } }
    };
}

impl_278!()