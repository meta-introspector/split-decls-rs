macro_rules! deps {
    () => {
        Equal!();
        IsGreaterPrivate!();
        B0!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < A , B > IsGreaterPrivate < B , Equal > for A { type Output = False ; # [inline] fn is_greater_private (self , _ : B , _ : Equal) -> Self :: Output { B0 } }
    };
}

impl_281!();