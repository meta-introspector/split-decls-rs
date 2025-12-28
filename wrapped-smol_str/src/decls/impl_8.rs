macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PartialEq < str > for SmolStr { # [inline (always)] fn eq (& self , other : & str) -> bool { self . as_str () == other } }
    };
}

impl_8!();