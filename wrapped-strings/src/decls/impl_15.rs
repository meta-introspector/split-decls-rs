macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl PartialEq < BSTR > for String { fn eq (& self , other : & BSTR) -> bool { other == self } }
    };
}

impl_15!();