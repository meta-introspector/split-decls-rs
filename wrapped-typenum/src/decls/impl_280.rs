macro_rules! deps {
    () => {
        Less!();
        IsGreaterPrivate!();
        B0!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < A , B > IsGreaterPrivate < B , Less > for A { type Output = False ; # [inline] fn is_greater_private (self , _ : B , _ : Less) -> Self :: Output { B0 } }
    };
}

impl_280!()