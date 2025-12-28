macro_rules! deps {
    () => {
        Equal!();
        B1!();
        IsGreaterOrEqualPrivate!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < A , B > IsGreaterOrEqualPrivate < B , Equal > for A { type Output = True ; # [inline] fn is_greater_or_equal_private (self , _ : B , _ : Equal) -> Self :: Output { B1 } }
    };
}

impl_293!()