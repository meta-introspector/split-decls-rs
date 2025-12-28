macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl PartialEq < String > for HSTRING { fn eq (& self , other : & String) -> bool { * self == * * other } }
    };
}

impl_40!();