macro_rules! deps {
    () => {
        IsLessOrEqualPrivate!();
        B0!();
        Greater!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < A , B > IsLessOrEqualPrivate < B , Greater > for A { type Output = False ; # [inline] fn is_less_or_equal_private (self , _ : B , _ : Greater) -> Self :: Output { B0 } }
    };
}

impl_286!();