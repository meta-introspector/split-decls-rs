macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl PartialEq < & str > for HSTRING { fn eq (& self , other : & & str) -> bool { * self == * * other } }
    };
}

impl_45!();