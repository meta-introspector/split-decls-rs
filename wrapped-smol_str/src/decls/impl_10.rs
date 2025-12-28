macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > PartialEq < & 'a str > for SmolStr { # [inline (always)] fn eq (& self , other : & & 'a str) -> bool { self == * other } }
    };
}

impl_10!()