macro_rules! deps {
    () => {
        B1!();
        IsLessPrivate!();
        Less!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < A , B > IsLessPrivate < B , Less > for A { type Output = True ; # [inline] fn is_less_private (self , _ : B , _ : Less) -> Self :: Output { B1 } }
    };
}

impl_272!()