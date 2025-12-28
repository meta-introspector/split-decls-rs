macro_rules! deps {
    () => {
        Less!();
        B1!();
        IsLessOrEqualPrivate!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < A , B > IsLessOrEqualPrivate < B , Less > for A { type Output = True ; # [inline] fn is_less_or_equal_private (self , _ : B , _ : Less) -> Self :: Output { B1 } }
    };
}

impl_284!()