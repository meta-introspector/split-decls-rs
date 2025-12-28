macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a > PartialEq < & 'a str > for Name { fn eq (& self , other : & & 'a str) -> bool { self == * other } }
    };
}

impl_17!()