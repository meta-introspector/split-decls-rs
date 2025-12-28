macro_rules! deps {
    () => {
        Greater!();
        IsGreaterOrEqualPrivate!();
        B1!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < A , B > IsGreaterOrEqualPrivate < B , Greater > for A { type Output = True ; # [inline] fn is_greater_or_equal_private (self , _ : B , _ : Greater) -> Self :: Output { B1 } }
    };
}

impl_294!();