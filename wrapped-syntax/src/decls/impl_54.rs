macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl PartialEq < TokenText < '_ > > for & '_ str { fn eq (& self , other : & TokenText < '_ >) -> bool { other == self } }
    };
}

impl_54!();