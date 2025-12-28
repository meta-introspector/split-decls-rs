macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl PartialEq < String > for Name { fn eq (& self , other : & String) -> bool { self . as_str () == other } }
    };
}

impl_90!();