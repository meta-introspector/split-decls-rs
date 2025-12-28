macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl PartialEq < BSTR > for & str { fn eq (& self , other : & BSTR) -> bool { other == self } }
    };
}

impl_14!();