macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > PartialEq < & 'a String > for SmolStr { # [inline (always)] fn eq (& self , other : & & 'a String) -> bool { self == * other } }
    };
}

impl_14!();