macro_rules! deps {
    () => {
        IsLessPrivate!();
        Greater!();
        B0!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < A , B > IsLessPrivate < B , Greater > for A { type Output = False ; # [inline] fn is_less_private (self , _ : B , _ : Greater) -> Self :: Output { B0 } }
    };
}

impl_274!()