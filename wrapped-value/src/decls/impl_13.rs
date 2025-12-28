macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl PartialEq < String > for Name { fn eq (& self , other : & String) -> bool { self . as_str () == other } }
    };
}

impl_13!()