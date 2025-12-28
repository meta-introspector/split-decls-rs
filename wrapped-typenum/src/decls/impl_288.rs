macro_rules! deps {
    () => {
        Less!();
        B1!();
        IsNotEqualPrivate!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < A , B > IsNotEqualPrivate < B , Less > for A { type Output = True ; # [inline] fn is_not_equal_private (self , _ : B , _ : Less) -> Self :: Output { B1 } }
    };
}

impl_288!();