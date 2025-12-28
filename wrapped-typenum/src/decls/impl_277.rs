macro_rules! deps {
    () => {
        Equal!();
        IsEqualPrivate!();
        B1!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < A , B > IsEqualPrivate < B , Equal > for A { type Output = True ; # [inline] fn is_equal_private (self , _ : B , _ : Equal) -> Self :: Output { B1 } }
    };
}

impl_277!();