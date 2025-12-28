macro_rules! deps {
    () => {
        IsGreaterOrEqualPrivate!();
        B0!();
        Less!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < A , B > IsGreaterOrEqualPrivate < B , Less > for A { type Output = False ; # [inline] fn is_greater_or_equal_private (self , _ : B , _ : Less) -> Self :: Output { B0 } }
    };
}

impl_292!()