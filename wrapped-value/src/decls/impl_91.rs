macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl PartialEq < str > for Name { fn eq (& self , other : & str) -> bool { self . as_str () == other } }
    };
}

impl_91!()