macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl PartialOrd < u128 > for Pu128 { # [inline] fn partial_cmp (& self , other : & u128) -> Option < Ordering > { { self . 0 } . partial_cmp (other) } }
    };
}

impl_355!();