macro_rules! deps {
    () => {
        Equal!();
        IsLessPrivate!();
        B0!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < A , B > IsLessPrivate < B , Equal > for A { type Output = False ; # [inline] fn is_less_private (self , _ : B , _ : Equal) -> Self :: Output { B0 } }
    };
}

impl_273!()