macro_rules! deps {
    () => {
        IsNotEqualPrivate!();
        B0!();
        Equal!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < A , B > IsNotEqualPrivate < B , Equal > for A { type Output = False ; # [inline] fn is_not_equal_private (self , _ : B , _ : Equal) -> Self :: Output { B0 } }
    };
}

impl_289!();