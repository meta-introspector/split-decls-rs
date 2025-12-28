macro_rules! deps {
    () => {
        Length!();
        Len!();
        UTerm!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        # [doc = " Length of `UTerm` by itself is 0"] impl Len for UTerm { type Output = U0 ; # [inline] fn len (& self) -> Self :: Output { UTerm } }
    };
}

impl_348!();