macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl PartialEq < TokenText < '_ > > for String { fn eq (& self , other : & TokenText < '_ >) -> bool { other == self } }
    };
}

impl_56!();