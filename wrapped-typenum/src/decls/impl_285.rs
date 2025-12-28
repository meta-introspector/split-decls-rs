macro_rules! deps {
    () => {
        IsLessOrEqualPrivate!();
        Equal!();
        B1!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < A , B > IsLessOrEqualPrivate < B , Equal > for A { type Output = True ; # [inline] fn is_less_or_equal_private (self , _ : B , _ : Equal) -> Self :: Output { B1 } }
    };
}

impl_285!();