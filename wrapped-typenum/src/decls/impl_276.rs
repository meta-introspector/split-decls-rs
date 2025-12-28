macro_rules! deps {
    () => {
        Less!();
        IsEqualPrivate!();
        B0!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < A , B > IsEqualPrivate < B , Less > for A { type Output = False ; # [inline] fn is_equal_private (self , _ : B , _ : Less) -> Self :: Output { B0 } }
    };
}

impl_276!()