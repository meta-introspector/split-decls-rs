macro_rules! deps {
    () => {
        ATerm!();
        Len!();
        Length!();
        UTerm!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        # [doc = " Length of `ATerm` by itself is 0"] impl Len for ATerm { type Output = U0 ; # [inline] fn len (& self) -> Self :: Output { UTerm } }
    };
}

impl_533!();