macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl PartialEq < u128 > for Pu128 { # [inline] fn eq (& self , other : & u128) -> bool { ({ self . 0 }) == * other } }
    };
}

impl_354!()