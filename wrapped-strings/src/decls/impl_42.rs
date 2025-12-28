macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl PartialEq < & String > for HSTRING { fn eq (& self , other : & & String) -> bool { * self == * * * other } }
    };
}

impl_42!();