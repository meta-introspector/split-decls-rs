macro_rules! deps {
    () => {
        IsGreaterPrivate!();
        Greater!();
        B1!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < A , B > IsGreaterPrivate < B , Greater > for A { type Output = True ; # [inline] fn is_greater_private (self , _ : B , _ : Greater) -> Self :: Output { B1 } }
    };
}

impl_282!();